use std::{
    error::Error,
    fs::{self, File as file},
    io::{BufRead, BufReader, Read, Write},
    net::{Shutdown, TcpStream},
    path::Path,
};

use crate::structs;
use structs::jsons::RequestNewShip;
use crate::structs::jsons::{Chunks, File, ReciverRespond};

fn ship_request(
    file_paths: &[String], 
    sender: &mut TcpStream, 
    id_shipment: &str
) -> Result<bool, Box<dyn Error>> {
    let mut files: Vec<File> = Vec::new();

    for file_path in file_paths {
        let file_name = Path::new(&file_path)
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("unknown_file");

        let size = fs::metadata(file_path)?.len();

        let file = File {
            file_name: file_name.to_string(),
            file_size: size,
        };

        files.push(file);
    }

    let request = RequestNewShip {
        files,
        id: id_shipment.to_string(),
    };

    let mut serialize_request = serde_json::to_string(&request)?;
    serialize_request.push('\n');

    sender.write_all(serialize_request.as_bytes())?;
    sender.flush()?;

    let mut buffer = BufReader::new(sender.try_clone()?);
    let mut response = String::new();

    buffer.read_line(&mut response)?;

    let reciver_response: ReciverRespond = serde_json::from_str(&response)?;

    Ok(reciver_response.state)
}

pub fn Tcp_sender(ip: &str, files_path: Vec<String>) -> Result<(), Box<dyn Error>> {
    let full_address = format!("{}:8080", ip);
    
    let mut sender = TcpStream::connect(full_address)?;

    let id = uuid::Uuid::new_v4().to_string();
    let continue_shipping = ship_request(&files_path, &mut sender, &id)?;

    if !continue_shipping {
        println!("The receiver rejected the offer");
        return Ok(());
    }

    for (index, file_path) in files_path.iter().enumerate() {
        let file = file::open(file_path)?;

        let file_name = Path::new(file_path)
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("unknown_file");

        println!("Sending: {}", file_name);
        let total_size = file.metadata()?.len();

        let mut buffer_file = BufReader::new(file);

        let mut read_bytes: u64 = 0;
        let mut cont: u16 = 1;

        let last_file_batch = index == files_path.len() - 1;

        while total_size > read_bytes {
            let mut buffer = [0u8; 65536];

            let bytes = buffer_file.read(&mut buffer)?;

            if bytes == 0 {
                break;
            }

            read_bytes += bytes as u64;

            let last_chunk_file = read_bytes >= total_size;

            let data = &buffer[..bytes];

            let json_chunk = Chunks {
                number: cont,
                content: data.to_vec(),
                id: id.to_string(),
                file_name: file_name.to_string(),
                last_chunk: last_chunk_file,
                last_file: last_file_batch,
            };

            let mut json_string = serde_json::to_string(&json_chunk)?;
            json_string.push('\n');

            sender.write_all(json_string.as_bytes())?;

            cont += 1;
        }
    }

    sender.flush()?;

    let mut buffer = String::new();
    let mut buffer_reader = BufReader::new(sender.try_clone()?);

    buffer_reader.read_line(&mut buffer)?;

    let respond: ReciverRespond = serde_json::from_str(&buffer)?;

    if respond.state {
        sender.shutdown(Shutdown::Write)?;
    }

    println!("All files have been sent successfully.");
    Ok(())
}