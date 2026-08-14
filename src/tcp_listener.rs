use std::fs::File;
use std::{io, net::TcpListener};
use std::io::{BufRead, BufReader, Write};
use crate::structs::jsons::{ReciverRespond, RequestNewShip, Chunks};

pub fn Tcp_listener() {
    let common_adress = "0.0.0.0:8080";
    let listener = TcpListener::bind(common_adress).expect("Error at binding address");

    loop {
        match listener.accept() {
            Ok((mut stream, adrss)) => {
                let mut buffer = BufReader::new(stream.try_clone().unwrap());

                let mut input_request = String::new();
                buffer.read_line(&mut input_request).unwrap();
                let json_request: RequestNewShip = serde_json::from_str(&input_request).expect("error at deserializing json");

                println!("file: {:?}, id: {}", &json_request.files, &json_request.id);
                println!("introduce yes if you want to accept the connection o no to reject it ");

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("error at reading the user output");

                let stt: bool;
                if input.trim() == "yes" {
                    println!("accepted");
                    stt = true;
                } else {
                    println!("connection reject it");
                    stt = false;
                }

                let response = ReciverRespond { state: stt };
                let mut response_string = serde_json::to_string(&response).unwrap();
                response_string.push_str("\n");

                stream.write_all(response_string.as_bytes()).unwrap();
                stream.flush().unwrap();

                if !stt {
                    continue;
                }

                let uuid = json_request.id;


                input_request.clear();
                buffer.read_line(&mut input_request).expect("error at reading the file");
                let mut json_chunk: Chunks = serde_json::from_str(&input_request).expect("error at converting the string to a struct");


                loop {
                    let mut open_file = File::create(json_chunk.file_name.to_owned()).expect("error while creating a new file");


                    loop {
                        if json_chunk.id == uuid {
                            open_file.write_all(&json_chunk.content).expect("msg");
                        }


                        if json_chunk.last_chunk {
                            break;
                        }


                        input_request.clear();
                        let result = buffer.read_line(&mut input_request).unwrap_or(0);
                        if result != 0 {
                            let new_chunk = serde_json::from_str(&input_request).expect("error parsing chunk");
                            json_chunk = new_chunk;
                        } else {
                            break;
                        }
                    }

                    open_file.flush().expect("Error flushing file to disk");


                    if json_chunk.last_file {
                        break;
                    }


                    input_request.clear();
                    let result = buffer.read_line(&mut input_request).unwrap_or(0);
                    if result != 0 {
                        json_chunk = serde_json::from_str(&input_request).expect("error parsing next file chunk");
                    } else {
                        break;
                    }
                }


                let all_file_reciver_respond = ReciverRespond { state: true };
                let mut string_respond = serde_json::to_string(&all_file_reciver_respond).expect("error serializing json");
                string_respond.push('\n');

                stream.write_all(string_respond.as_bytes()).unwrap();
                stream.flush().unwrap();

                println!("all files had been recived ");
            }
            Err(e) => println!("couldn't get client: {}", e),
        }
    }
}