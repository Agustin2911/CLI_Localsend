mod structs;
mod reciver;
mod sender;
const ASCII_ART: &str = r#"
  ____ _     ___   _                     _                    _
 / ___| |   |_ _| | |    ___   ___  __ _| |___  ___ _ __   __| |
| |   | |    | |  | |   / _ \ / __|/ _` | / __|/ _ \ '_ \ / _` |
| |___| |___ | |  | |__| (_) | (__| (_| | \__ \  __/ | | | (_| |
 \____|_____|___| |_____\___/ \___|\__,_|_|___/\___|_| |_|\__,_|
"#;

use clap::{Parser, Subcommand};
use crate::reciver::listener::listener_;
use crate::sender::sender::sender_;

use crate::structs::common_functions::{ write_name, write_path};
#[derive(Parser)]
#[command(name = "localsend")]
#[command(about = "A cli version of localsend", long_about = None)]
#[command(before_help = ASCII_ART)]
struct Cli {
    #[command(subcommand)]
    comando: Comandos,
}

#[derive(Subcommand)]
enum Comandos {
    

    #[command(name = "-l to listen to incoming requests", alias = "-l")]
    Listen,

  
    #[command(name = "-s to send files", alias = "-s")]
    Send ,
    

    #[command(name = "-c to config your user \
    [arguments] \
    -n [name] to set your name\
    -p [path] to your save folder", alias = "-c")]
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
            println!("Initalizing the listener mode...");
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
                println!("please use at least one of this arguments (-n or -p)");
            }
        }
    }
}

