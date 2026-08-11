
use std::{arch::x86_64, io, net::TcpListener};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use crate::structs::jsons::{first_respond_reciver, request_new_ship};

pub fn Tcp_listener(){

    let common_adress= "0.0.0.0:8080";

    let listener=TcpListener::bind(common_adress);


    match listener.unwrap().accept() {


        Ok((mut stream, adrss)) => {

            let mut buffer= BufReader::new(stream.try_clone().unwrap());

            let mut input_request=String::new();
            buffer.read_line(&mut input_request).unwrap();
            let json_request:request_new_ship=serde_json::from_str(&input_request).expect("error at deserializing json");

            println!("file: {:?}, size: {}, id: {}",&json_request.file_names,&json_request.complete_size,&json_request.id);

            println!("introduce yes if you want to accept the connection o no to reject it ");

            let mut input = String::new();

            io::stdin().read_line(&mut input);

            let mut stt:bool;
            if input.trim() == "yes" {

                println!("accepted");
                stt = true;
            }
            else {
                println!("connection reject it");
                stt=false;
            }


            let response=first_respond_reciver{
                state:stt
            };

            let mut response_string=serde_json::to_string(&response).unwrap();

            response_string.push_str("\n");


            stream.write_all(response_string.as_bytes()).unwrap();

            stream.flush().unwrap();

            if !stt{
                return;
            }

            let uuid=json_request.id;



        }
        Err(e) => println!("couldn't get client: {}", e),

    }


}
