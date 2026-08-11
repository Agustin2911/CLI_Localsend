use std::{ fs::{self, File as file}, io::{ BufReader, Read, Write}, net::TcpStream, ops::Add};
use std::io::BufRead;

use std::path::Path;

use structs::jsons::RequestNewShip;

use crate::structs;
use crate::structs::jsons::{Chunks, FirstRespondReciver,File};



fn  ship_request(file_paths:&Vec<String>, sender:&mut TcpStream,id_shipment:&str)-> bool{


    let mut files:Vec<File>=Vec::new();


    for file_path in file_paths{
        let file_name = Path::new(&file_path)
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("unknow_file");

        let size=fs::metadata(file_path).expect("").len();
        
        let file=File{
            file_name:file_name.to_string(),
            file_size:size
        };

        files.push(file);

    }

    let request= RequestNewShip{

        files:files,
        id: id_shipment.to_string()
    };


    let mut  serialize_request=serde_json::to_string(& request).expect("error at serializing request to json");

    serialize_request.push_str("\n");

    sender.write_all(serialize_request.as_bytes()).expect("error at writing json");

    sender.flush().expect("error at flushing json");


    let mut buffer= BufReader::new(sender.try_clone().expect("msg"));

    let mut response= "".to_string();

    buffer.read_line(&mut response).expect("error at reading json");

    let reciver_response:FirstRespondReciver=serde_json::from_str(&response).expect("error at deserializing json");

    reciver_response.state

}




pub fn Tcp_sender(ip:&str, files_path:Vec<String>) {
    let full_address = ip.to_string() + ":8080";
    let mut sender = TcpStream::connect(full_address).expect("Error at creating the tcp connection");


    let id = uuid::Uuid::new_v4().to_string();
    let continue_shipping = ship_request(&files_path,  &mut sender, &id);

    if !continue_shipping {
        println!("The reciver reject the offer");
        return;
    }

    for file_path in files_path {
        let file = file::open(&file_path).unwrap();
        
          let file_name = Path::new(&file_path)
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("unknow_file");


        let total_size = file.metadata().unwrap().len();

        let mut buffer_file = BufReader::new(file);

        let mut read_bytes = 0;
        let mut cont: u16 = 1;

        while total_size > read_bytes {
            let mut buffer = [0u8; 65536];

            let bytes = buffer_file.read(&mut buffer).unwrap();

            let data = &buffer[..bytes];

            let json_chunk = Chunks {
                number: cont,
                content: data.to_vec(),
                id: id.to_string(),
                file_name:file_name.to_string()
            };

            let mut json_string = serde_json::to_string(&json_chunk).expect("error at serializing json");

            json_string.push('\n');

            sender.write_all(json_string.as_bytes()).expect("error at writing json");

            

            cont = cont + 1;
            read_bytes += bytes as u64;
        }
        sender.flush().expect("msg");
    }
}