use std::fs::File;
use std::{io, net::{TcpListener, TcpStream}};
use std::io::{BufRead, BufReader, Write};
// Asumiendo que estos imports existen en tu proyecto
use crate::structs::jsons::{ReciverRespond, RequestNewShip, Chunks};
use crate::structs::common_functions::input_validation;

pub fn Tcp_listener(path : &str) {
    let common_address = "0.0.0.0:8080";

    let listener = match TcpListener::bind(common_address) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error binding address: {}", e);
            return;
        }
    };

    println!("Listening in  {}", common_address);

    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                println!("New connection from: {}", addr);
                if let Err(e) = handle_client(stream,path) {
                    eprintln!("Error  {}: {}", addr, e);
                }
            }
            Err(e) => eprintln!("Error  {}", e),
        }
    }
}

fn handle_client(mut stream: TcpStream,path : &str) -> io::Result<()> {
    let mut buffer = BufReader::new(stream.try_clone()?);
    let mut input_request = String::new();

    if buffer.read_line(&mut input_request)? == 0 {
        return Err(io::Error::new(io::ErrorKind::ConnectionAborted, "Client disconnected"));
    }

    let json_request: RequestNewShip = serde_json::from_str(&input_request)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("JSON Error: {}", e)))?;

    println!("file: {:?}, id: {}", &json_request.files, &json_request.id);
    println!("Introduce 'yes' to accept the incoming files o 'no' to reject it :");

    let mut input;
    let mut  stt;
    loop {
        input = String::new();
        input_validation(&mut input);

        stt = input.trim() == "yes";
        if stt {
            println!("incoming files accepted");
            break;
        } else if input.trim() != "no" {
            println!("Wrong answer , introduce yes or no");

        } else {
            println!("incoming files rejected ");
            break;
        }

    }
    let response = ReciverRespond { state: stt };
    let mut response_string = serde_json::to_string(&response)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    response_string.push('\n');

    stream.write_all(response_string.as_bytes())?;
    stream.flush()?;

    if !stt {
        return Ok(()); 
    }

    let uuid = json_request.id;
    input_request.clear();

    if buffer.read_line(&mut input_request)? == 0 {
        return Err(io::Error::new(io::ErrorKind::ConnectionAborted, "Client disconnected abruptly"));
    }

    let mut json_chunk: Chunks = serde_json::from_str(&input_request)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    loop {
        let mut open_file = File::create(path.to_string()+"/"+&json_chunk.file_name)?;

        loop {
            if json_chunk.id == uuid {
                open_file.write_all(&json_chunk.content)?;
            }

            if json_chunk.last_chunk {
                break;
            }

            input_request.clear();
            if buffer.read_line(&mut input_request)? == 0 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Incomplete chunk transmission"));
            }
            
            json_chunk = serde_json::from_str(&input_request)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        }

        open_file.flush()?;

        if json_chunk.last_file {
            break;
        }

        input_request.clear();
        if buffer.read_line(&mut input_request)? == 0 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Incomplete file transmission"));
        }

        json_chunk = serde_json::from_str(&input_request)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    }

    let all_file_reciver_respond = ReciverRespond { state: true };
    let mut string_respond = serde_json::to_string(&all_file_reciver_respond)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    string_respond.push('\n');

    stream.write_all(string_respond.as_bytes())?;
    stream.flush()?;

    println!("All the files have been recived");
    
    Ok(())
}