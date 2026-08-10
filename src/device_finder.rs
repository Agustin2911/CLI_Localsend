use zeroconf::{ServiceType, prelude::*};
use zeroconf::{MdnsService,MdnsBrowser};
use std::collections::HashMap;
use std::time::Duration;
use std::sync::{Arc,Mutex};

pub fn Device_finder()-> HashMap<String, String>{


    let  mut users:Arc< Mutex< HashMap<String,String >>>=Arc::new(Mutex::new(HashMap::new()));

    let users_clone=users.clone();

    let service_type= ServiceType::new("clilocalsend","tcp").expect("error at the creation of the service type");


    let mut browser= MdnsBrowser::new(service_type);

    browser.set_service_discovered_callback(Box::new(move |resultado, _contexto| {
        match resultado {
            Ok(servicio) => {
                
                if let Ok(mut map) = users_clone.lock() {

                    if servicio.address().to_string().split(".").collect::<Vec<&str>>()[0]=="192"{
                    
                         map.insert(servicio.name().to_string(), servicio.address().to_string()); 
                    }
                    
                   
                }
                
            }
            Err(e) => eprintln!("Error al descubrir servicio: {:?}", e),
        }
    }));


    let service_loop= browser.browse_services().expect("error at the declaration of the browser");

    for i in 0..50{
        service_loop.poll(Duration::from_millis(100)).expect("error in the loop of the browser");
    }

    users.lock().unwrap().clone()

}