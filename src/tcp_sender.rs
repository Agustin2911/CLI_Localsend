use std::{fs::File, io::{self, BufReader, Read, Write}, net::TcpStream};
use std::io::BufRead;
use std::ops::Add;
use std::path::Path;

use structs::jsons::request_new_ship;
use uuid::{uuid, Uuid};
use crate::structs;
use crate::structs::jsons::{chunks, first_respond_reciver};

fn  ship_request(file_paths:&Vec<String>, size:u64, sender:&mut TcpStream,id_shipment:&str)-> bool{

    let mut list_names:Vec<String>=Vec::new();

    for file_path in file_paths{
        let file_name = Path::new(&file_path)
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("unknow_file");
        list_names.push(file_name.to_string());
    }

    let request= request_new_ship{

        file_names:list_names,
        complete_size:size,
        id: id_shipment.to_string()
    };


    let mut  serialize_request=serde_json::to_string(& request).expect("error at serializing request to json");

    serialize_request.push_str("\n");

    sender.write_all(serialize_request.as_bytes()).expect("error at writing json");

    sender.flush().expect("error at flushing json");


    let mut buffer= BufReader::new(sender);

    let mut response= "".to_string();

    buffer.read_line(&mut response).expect("error at reading json");

    let reciver_response:first_respond_reciver=serde_json::from_str(&response).expect("error at deserializing json");

    reciver_response.state

}

pub fn Tcp_sender(ip:&str, files_path:Vec<String>) {
    let full_address = ip.to_string() + ":8080";
    let mut sender = TcpStream::connect(full_address).expect("Error at creating the tcp connection");


    let id = uuid::Uuid::new_v4().to_string();
    let continue_shipping = ship_request(&files_path, 100, &mut sender, &id);

    if !continue_shipping {
        println!("The reciver reject the offer");
        return;
    }

    for file_path in files_path {
        let file = File::open(file_path).unwrap();

        let total_size = file.metadata().unwrap().len();

        let mut buffer_file = BufReader::new(file);

        let mut read_bytes = 0;
        let mut cont: u16 = 1;

        while total_size > read_bytes {
            let mut buffer = [0u8; 65536];

            let bytes = buffer_file.read(&mut buffer).unwrap();

            let data = &buffer[..bytes];

            let json_chunk = chunks {
                number: cont,
                content: data.to_vec(),
                id: id.to_string()
            };

            let json_string = serde_json::to_string(&json_chunk).expect("error at serializing json");

            sender.write_all(json_string.as_bytes()).expect("error at writing json");

            sender.flush().expect("error at flushing json");

            cont = cont + 1;
            read_bytes += bytes as u64;
        }
    }
}