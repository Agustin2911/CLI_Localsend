

use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize,Deserialize,Debug)]
pub struct  RequestNewShip{
    pub files: Vec<File>,
    pub id:String
}

#[derive(Serialize,Deserialize,Debug)]
pub struct FirstRespondReciver{
    pub state:bool
}


#[derive(Serialize,Deserialize,Debug)]
pub struct Chunks{

    pub number:u16,
    pub content:Vec<u8>,
    pub id:String,
    pub file_name:String
}

#[derive(Serialize,Deserialize,Debug)]
pub struct File {

    pub file_name:String,
    pub file_size:u64


}