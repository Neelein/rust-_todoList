use super::todo_model::Todo;


#[derive(Debug)]
pub struct MyLsit{
    pub data:Vec<Todo>
}


#[allow(dead_code)]
impl MyLsit {
    pub fn add_to_list(&self){
        println!("add something");
    }
}