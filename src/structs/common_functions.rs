use std::{ffi::FromBytesUntilNulError, fs::{File, create_dir_all}, io::{self, BufReader, BufWriter, Write}, ops::Add, path::Path};

use dirs::config_dir;
use serde::de::value::Error;

use crate::sender::tcp_sender::Tcp_sender;
use crate::sender::device_finder::Device_finder;
use crate::structs::jsons::Json;

use std::{collections::HashMap, env, fs::{self}, path::PathBuf};


pub fn select_a_file() -> String{

    let mut selected_file_path:String=String::new();


    let home_path = match env::var("HOME") {

        Ok(result)=> result,
        Err(error) => {
            println!("Error at reading the HOME path, error name {}",error);
            return String::new();
        }
        
    };




     
    let mut path=PathBuf::from(home_path);

    path.push("Documentos");

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

    let  input:u16;
    loop {

        input_validation(&mut input_text);


        match input_text.trim().parse::<i16>(){

            Ok(r)=>{
                input= r as u16;
                break;
            },
            Err(e)=> {
                println!("The input value was a string , please enter only numbers,error name : {}",e);
                input_text="".to_string();
            }
            
        }
        
        
    }

    
        

    match  list_of_files.get(&input){
        
        Some((name,true))=>{

            path.push(name);
        }

        Some((name,false))=>{

            path.push(name);

            selected_file_path=path.to_string_lossy().into_owned();

        }

        None => {
                println!("Error: the number doesn't exist.");
            }

   
}

}


    println!("{}",selected_file_path);

    selected_file_path

}







pub fn select_reciver()-> String{



    let devices=Device_finder();
    
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
    input_validation(&mut input);

    let number= input.trim().parse::<u16>().expect("error at parsing the string");

    cont=1;
    for i in &devices{

        if cont==number{
            return i.1.to_owned();
        }
    }

    return "no device found".to_owned();

}


pub fn send_a_file(){



    let mut files: Vec<String>= Vec::new();

    let mut option=String::new();


    while  option.trim()!="stop"{

        let selected_file=select_a_file();

        if !selected_file.is_empty(){

            files.push(selected_file);
        }
        println!("enter any key to continue or enter stop to finish with the file selection");
        option.clear();

        input_validation(&mut option);

    }

    let mut device=select_reciver();

    let mut continue_searching="si".to_owned();

    while device=="no device found" && continue_searching!="no"{


        device=select_reciver();

        if device=="no device found"{
            
           input_validation(&mut continue_searching);
        }


    }

    if continue_searching=="no"{
        return;
    }



    Tcp_sender(&device, files);

}




pub fn input_validation (input:&mut String ){


    loop {
         
        input.clear();

        match io::stdin().read_line( input) {
            

            Ok(0)=>{
                break;
            }

            Ok(r)=>{
                break;
            }

            Err(e)=> {

                println!("Input error , error name: {}",e);

            }
        }

    }



}


pub fn read_config()-> (Json,String){


    let mut  home_directory= match env::consts::OS {
        
        "linux"| "macos"=>{

            let env=env::var("HOME").expect("There is no HOME variable");
            let mut path=PathBuf::from(env);
            path.push(".config");

            path

        }

        "windows"=>{

            let env=env::var("APPDATA").expect("there is no APPDATA variable");
            let path= PathBuf::from(env);

            path
    
        }


        _ =>{
            panic!("the os is not reconizable")
        }
    };


    home_directory.push("cli_localsend");
    home_directory.push("config.json");
    
    let config_path= Path::new(&home_directory);

    

    if config_path.exists() {

    

        match File::open(config_path){

         Ok(r)=>{

            let buffer= BufReader::new(r);

            let result_json:Json= serde_json::from_reader(buffer).expect("msg");


             (result_json,config_path.to_string_lossy().to_string())

        }
        Err(e)=>{

            
        (Json{ name:"user".to_string(),  path:dirs::home_dir().expect("").to_string_lossy().to_string() },config_path.to_string_lossy().to_string())

    }

    }


    }
    else{
        
        if let Some(path)= config_path.parent(){

            create_dir_all(path).expect("error at creating the directory");
        }


        let content=Json{ name:"user".to_string(),  path:dirs::home_dir().expect("").to_string_lossy().to_string() };
        let content_string= serde_json::to_string(&content).expect("error at converting the json to string");
        fs::write(config_path,content_string).expect("error at creating the config.json");

        (Json{ name:"user".to_string(),  path:dirs::home_dir().expect("").to_string_lossy().to_string() },config_path.to_string_lossy().to_string())
    }
}


pub fn write_name(name:&str){
    let mut json=read_config();

    

    json.0.change_name(name.to_string());
    let json_string=serde_json::to_string(&json.0).expect("error at converting ");
    

    fs::write(json.1,json_string).expect("msg");

    
    show_config();
}

pub fn write_path(path:&str){

    let mut json=read_config();

   

    json.0.change_path(path.to_string());
     let json_string=serde_json::to_string(&json.0).expect("error at converting ");

    fs::write(json.1,json_string);

    show_config();

}

fn show_config(){


    let json=read_config();
    println!("Current config: name: {} save to folder: {}",json.0.name,json.0.path);


}


