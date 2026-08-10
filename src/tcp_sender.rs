use std::{fs::File, io::{self, BufReader, Read, Write}, net::TcpStream};

pub fn Tcp_sender(ip:&str , file_path:String){

    let full_address= ip.to_string()+ ":8080";
    let mut  sender=TcpStream::connect(full_address).expect("Error at creating the tcp connection");

    
    let open_file=File::open(&file_path).expect("error occur while opening a file");

    let total=open_file.metadata().expect("").len();

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


}