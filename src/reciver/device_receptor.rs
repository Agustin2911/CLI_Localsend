
use zeroconf::{ServiceType,  prelude::*};
use zeroconf::{MdnsService};
use std::time::Duration;


pub fn Device_receptor(port: u16,device_name:&str) {
    
    let service_type = match ServiceType::new("clilocalsend", "tcp"){

            Ok(r)=>r,

            Err(e)=>{
                println!("Error at the creation of the service type, error : {}",e);
                return;
            }


    };


    let mut service = MdnsService::new(service_type, port);


    service.set_name(device_name);

    service.set_registered_callback(Box::new(|_, _| {
        println!("listener up!");
    }));


    let running_service = match service.register(){

        Ok(r)=>r,
        Err(e)=>{

            println!("error at the registration of the service, {}",e);
            return;
        }
    };

    loop {
        
    if let Err(e)=running_service.poll(Duration::from_millis(100)){

        println!("error : {}",e);
    }
    
    }

}