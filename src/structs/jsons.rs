use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize,Deserialize,Debug)]
pub struct  request_new_ship{
    pub file_name: String,
    pub complete_size:u64,
    pub id:String
}

#[derive(Serialize,Deserialize,Debug)]
pub struct first_respond_reciver{
    state:bool
}

/*
#[derive(Serialize,Deserialize,Debug)]
pub struct chunks{

    number:u16,
    content:[0u8;65536]
}*/
