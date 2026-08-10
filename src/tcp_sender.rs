use std::{fs::File, io::{self, BufReader, Read, Write}, net::TcpStream};
use std::io::BufRead;
use std::path::Path;
use structs::jsons::request_new_ship;
use uuid::uuid;
use crate::structs;


pub fn Tcp_sender(ip:&str, file_path:String){

    let full_address= ip.to_string()+ ":8080";
    let mut  sender=TcpStream::connect(full_address).expect("Error at creating the tcp connection");

    let file_name = Path::new(&file_path)
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("unknow_file");

    let open_file=File::open(&file_path).expect("error occur while opening a file");

    let total=open_file.metadata().expect("").len();

    let request= request_new_ship{

        file_name:file_name.to_string(),
        complete_size:total,
        id: uuid::Uuid::new_v4().to_string()

    };

    let serialize_request=serde_json::to_string(& request).expect("error at serializing request to json");

    sender.write_all(serialize_request.as_bytes()).expect("error at writing json");

    sender.flush().expect("error at flushing json");


    let mut answer=String::new();
    let mut buffer_response= BufReader::new(BufReader::new(&sender));

    buffer_response.read_line(&mut answer).expect("error at reading json");

    let response=serde_json::to_string(&answer);

    println!("{:?}",response);

    /*
    let mut buffer= BufReader::new(open_file);

    let mut chunk= [0u8;65536];


    let mut read_byte:usize=1;

    let mut total_bytes_sended:u64=0;

    while read_byte!= (0 as usize) {
        

        read_byte=buffer.read(&mut chunk).expect("");

        sender.write_all(&chunk[..(read_byte as usize)]);


        total_bytes_sended+=&(read_byte as u64);

        println!(" porcentage: {}", (total_bytes_sended as f64)/(total as f64) );
        io::stdout().flush();
    }

    sender.flush().expect("error at flush");

    */
}