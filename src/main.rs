mod structs;
mod reciver;
mod sender;


use clap::{Parser, Subcommand};
use crate::reciver::listener::listener_;
use crate::sender::sender::sender_;

use crate::structs::common_functions::{ write_name, write_path};
#[derive(Parser)]
#[command(name = "localsend")]
#[command(about = "A cli version of localsend", long_about = None)]
struct Cli {
    #[command(subcommand)]
    comando: Comandos,
}

#[derive(Subcommand)]
enum Comandos {
    

    #[command(name = "listen", alias = "-l")]
    Listen,

  
    #[command(name = "send", alias = "-s")]
    Send ,
    

    #[command(name = "config", alias = "-c")]
    Config {
        
        #[arg(short = 'n', long = "name")]
        name: Option<String>,

      
        #[arg(short = 'p', long = "path")]
        path: Option<String>,
    },
}

fn main() {
    
    let cli = Cli::parse();

   
    match &cli.comando {
        Comandos::Listen => {
            println!("Iniciando el modo receptor (Listener)...");
            listener_();
        }
        Comandos::Send  => {
         
            sender_();
           
        }
        Comandos::Config { name, path } => {
            

           
            
            if let Some(n) = name {
                
                write_name(n);
            }
            if let Some(p) = path {
                write_path(p);
            }
            
            if name.is_none() && path.is_none() {
                println!("No se proporcionó ni -n ni -p. Usa 'localsend config --help' para ver las opciones.");
            }
        }
    }
}