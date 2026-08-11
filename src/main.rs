mod device_receptor;
mod device_finder;
mod tcp_sender;
mod tcp_listener;
pub mod structs;


use tcp_sender::Tcp_sender;


use std::{collections::HashMap, env, fs::{self, File}, io::{self, BufRead, BufReader, Read}, path::PathBuf, thread::spawn};
use std::thread;
use device_receptor::Device_receptor;
use tcp_listener::Tcp_listener;


fn select_a_file() -> String{

    let mut selected_file_path:String=String::new();


     
    let mut path=PathBuf::from(env::var("HOME").expect("error"));


    while selected_file_path=="" {
        
    let mut list_of_files: HashMap<u16, (String,bool)> =HashMap::new();


    if let Ok (files)=fs::read_dir(&path){

        let mut cont:u16=0;

        for i in files{



            if let Ok(file)=i{

                if let Ok(file_type)= &file.file_type(){

                    if file_type.is_dir(){

                        println!("{}-> 📂 directory: {:?}",cont,&file.file_name());
                        
                    }
                    else {
                        println!("{}-> 📄 file: {:?}",cont,&file.file_name())
                    }


                       list_of_files.insert(cont, (file.file_name().to_string_lossy().into_owned(),file_type.is_dir()));
                }

             

            }

            
            cont+=1;


        }

    }

    let mut input_text:String=String::new();

    io::stdin().read_line(&mut input_text).expect("error while reading the input data");


    if let Ok(input)=input_text.trim().parse::<u16>(){

    


    match  list_of_files.get(&input){
        
        Some((name,true))=>{

            path.push(name);
        }

        Some((name,false))=>{

            path.push(name);

            selected_file_path=path.to_string_lossy().into_owned();

        }

        None => {
                println!("Error: El número ingresado no existe en la lista.");
            }

      
    }
   
}


}


    println!("{}",selected_file_path);

    selected_file_path

}

fn select_reciver()-> String{


    let port:u16=8080;
    let name:String=String::from("capi_main");

    spawn(move ||{

        Device_receptor(port,&name)
    });

    let devices=device_finder::Device_finder();
    
    let mut cont:u16=1;

    if devices.len()==0{
        return "no devices found".to_owned();
    }

    for i in &devices{

        println!("{}->{}, {}",cont,i.0,i.1);

        cont+=1;
    }


    let mut input:String=String::new();

    println!("select the device");
    io::stdin().read_line(&mut input).expect("errro at reading the selected device");

    let number= input.trim().parse::<u16>().expect("error at parsing the string");

    cont=1;
    for i in &devices{

        if cont==number{
            return i.1.to_owned();
        }
    }

    return "no device found".to_owned();

}


fn send_a_file(){



    let mut files: Vec<String>= Vec::new();

    let mut option=String::new();


    while  option.trim()!="stop"{

        let selected_file=select_a_file();

        if !selected_file.is_empty(){

            files.push(selected_file);
        }
        println!("enter any key to continue or enter stop to finish with the file selection");
        option.clear();
        io::stdin().read_line(&mut option).expect("error in the input");

    }

    let mut device=select_reciver();

    let mut continue_searching="si".to_owned();

    while device=="no device found" && continue_searching!="no"{


        device=select_reciver();

        if device=="no device found"{
            
            io::stdin().read_line(&mut continue_searching).expect("error at the input");
        }


    }

    if continue_searching=="no"{
        return;
    }



    Tcp_sender(&device, files);

}

fn main() {

    thread::spawn(move || {
        Tcp_listener();
    } );
    send_a_file();



}
