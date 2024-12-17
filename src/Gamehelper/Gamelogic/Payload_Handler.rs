use std::any::Any;
use std::collections::HashMap;


pub struct Payload {
    name: String,
    content: HashMap<String, Box<dyn Any>>
}

impl Payload{
    pub fn new(name: String, content: HashMap<String, Box<dyn Any>>) -> Self {
        Self {
            name,
            content
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_content(&self) -> &HashMap<String, Box<dyn Any>>{
        &self.content
    }
}