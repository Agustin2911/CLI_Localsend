use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize,Deserialize,Debug)]
pub struct  request_new_ship{
    pub file_names: Vec<String>,
    pub complete_size:u64,
    pub id:String
}

#[derive(Serialize,Deserialize,Debug)]
pub struct first_respond_reciver{
    pub state:bool
}


#[derive(Serialize,Deserialize,Debug)]
pub struct chunks{

    pub number:u16,
    pub content:Vec<u8>,
    pub id:String
}
