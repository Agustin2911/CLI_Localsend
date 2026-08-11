
use std::fs::File;
use std::{ io, net::TcpListener};
use std::io::{BufRead, BufReader, Write};
use crate::structs::jsons::{FirstRespondReciver, RequestNewShip,Chunks};

pub fn Tcp_listener(){

    let common_adress= "0.0.0.0:8080";

    let listener=TcpListener::bind(common_adress);

    
    match listener.unwrap().accept() {


        Ok((mut stream, adrss)) => {

            let mut buffer= BufReader::new(stream.try_clone().unwrap());

            let mut input_request=String::new();
            buffer.read_line(&mut input_request).unwrap();
            let json_request:RequestNewShip=serde_json::from_str(&input_request).expect("error at deserializing json");

            
            println!("file: {:?}, id: {}",&json_request.files,&json_request.id);

            println!("introduce yes if you want to accept the connection o no to reject it ");

            let mut input = String::new();

            io::stdin().read_line(&mut input).expect("error at reading the user output");

            let  stt:bool;

            if input.trim() == "yes" {

                println!("accepted");
                stt = true;
            }
            else {
                println!("connection reject it");
                stt=false;
            }


            let response=FirstRespondReciver{
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

            let mut cont=0;


            input_request.clear();
            buffer.read_line( &mut input_request).expect("error at reading the file");
            

            let mut json_chunk:Chunks=serde_json::from_str(&input_request).expect("error at converting the string to a struct");
            
            let  mut actual_file=json_chunk.file_name.to_owned();

            print!("aca");
            while json_request.files.len()>cont{

                println!("primer while");
                let mut open_file=File::create(json_chunk.file_name.to_owned()).expect("error while creating a new file");
                
                
                while  json_chunk.file_name==actual_file {
                    
                    println!("segundo while");
                    if json_chunk.id==uuid{


                        println!("entro");
                        open_file.write_all(&json_chunk.content).expect("msg");    
                    
                    }

                    input_request.clear();
                    if buffer.read_line(&mut input_request).unwrap_or(0)==0{
                        break;
                    }

                    if let Ok(new_chunk) = serde_json::from_str(&input_request) {
                            json_chunk = new_chunk;
                    } else {
                        break; 
                    }


                }

                actual_file=json_chunk.file_name.to_owned();
                cont=cont+1;

            }
            



        }
        Err(e) => println!("couldn't get client: {}", e),

    }


}
