
use std::{arch::x86_64, io, net::TcpListener};
use std::io::{BufReader, BufWriter, Read, Write};

pub fn Tcp_listener(){

    let common_adress= "0.0.0.0:8080";

    let listener=TcpListener::bind(common_adress);

    match listener.unwrap().accept() {


        Ok((mut stream, adrss)) => {
            println!("{}", adrss);
            println!("introduce yes if you want to accept the connection o no to reject it ");

            let mut input = "yes";

            if input.trim() == "yes" {

                println!("accepted");

                let mut buffer = [0u8; 65536];

                let bytes_leidos=stream.read(&mut  buffer ).expect("");
                let bytes_array=String::from_utf8_lossy(&buffer[..bytes_leidos]);

                for i in bytes_array.lines(){
                    println!("{}", i);
                }
            }
            else {
                println!("connection reject it")
            }
    }
        Err(e) => println!("couldn't get client: {}", e),

    }


}
