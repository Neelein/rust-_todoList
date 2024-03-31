use chrono::prelude::*;

#[allow(dead_code)]
#[derive(Debug)]
pub struct  Todo{
    pub date_time: DateTime<chrono::Utc>,
    pub name:String,
    pub id: u32,
    pub state:String
}   


#[allow(dead_code)]
pub fn test_date_time(){
    let utc = Utc::now();

    println!("{}",utc);
}