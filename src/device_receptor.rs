
use zeroconf::{ServiceType,  prelude::*};
use zeroconf::{MdnsService};
use std::time::Duration;


pub fn Device_receptor(port: u16,device_name:&str) {
    let service_type = ServiceType::new("clilocalsend", "tcp").expect("Error at the creation of the service type");


    let mut service = MdnsService::new(service_type, port);


    service.set_name(device_name);

    service.set_registered_callback(Box::new(|_, _| {
        println!("✅ Servicio registrado en Avahi. Escuchando en la red...");
    }));


    let running_service = service.register().expect("error at the registration of the service");

    loop {
        
    running_service.poll(Duration::from_millis(100)).expect("error in the loop")
    }

}