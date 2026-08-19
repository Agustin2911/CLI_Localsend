use zeroconf::{ServiceType, prelude::*};
use zeroconf::{MdnsBrowser};
use std::collections::HashMap;
use std::time::Duration;
use std::sync::{Arc,Mutex};

pub fn Device_finder()-> HashMap<String, String>{


    let users:Arc< Mutex< HashMap<String,String >>>=Arc::new(Mutex::new(HashMap::new()));

    let users_clone=users.clone();

    
    let service_type= match ServiceType::new("clilocalsend","tcp") {
        
        Ok(r)=> r,

        Err(e)=>{

            println!("error at creating the service type: {}",e);
            return users.lock().unwrap().clone();
        }
        
    };


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



    let service_loop= match  browser.browse_services() {
        
        Ok(r)=> r,
        Err(e)=>{

            println!("error at creating the service browser, error : {}",e);
            return users.lock().unwrap().clone();
        }

    };
    
    for i in 0..50{
        
        if let Err(e)=service_loop.poll(Duration::from_millis(100)){
            println!("error : {}",e);
        }
    }

    users.lock().unwrap().clone()

}