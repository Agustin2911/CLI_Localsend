use std::thread::{self, spawn};

use crate::reciver::device_receptor::Device_receptor;
use crate::reciver::tcp_listener::Tcp_listener;
use crate::structs::common_functions::read_config;
pub fn listener_() {

    let port:u16=8080;
    
    let config=read_config();

    println!("{},{}",config.0.name,config.0.path);
    let device_receptor=spawn(move ||{
        Device_receptor(port,&config.0.name);
    });

    let tcp=thread::spawn(move || {
        Tcp_listener(&config.0.path);
    } );

    tcp.join().unwrap();
    device_receptor.join().unwrap();



}
