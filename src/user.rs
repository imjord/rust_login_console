use std::fs::File;
use std::io::prelude::*;
use serde_json::{Value};

pub struct User {
   pub username: String,
    pub password: String,
    pub logged_in: bool
}

impl User {
   pub fn new_user(username: String, password: String) -> User {
        User {
            username, 
            password, 
            logged_in: true
        }
    }
 
    pub fn login(&self) -> std::io::Result<()>{
    
        let mut user_file = File::open("user.json")?;
        let mut user_data = String::new();
      
        user_file.read_to_string(&mut user_data)?;
    
        let user_data_json: Value = serde_json::from_str(&user_data)?;
    
        if user_data_json["username"] == self.username && user_data_json["password"] == self.password {
            println!(" Logged in... Welcome {}", self.username)
        } else {
            println!(" Invalid credentials!") 
        }
    
        Ok(())
    }
   
}