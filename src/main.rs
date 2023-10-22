mod user;
mod create_user_file;
use user::User;
use std::io;
use create_user_file::create_user_file;
use std::path::Path;


fn main() {

    // check if theres a user.json 
    let user_path = Path::new("user.json").is_file();
  
    println!("Username... ");

    let mut user_name = String::new();

    io::stdin()
    .read_line(&mut user_name)
    .expect("Failed to read line");

    println!("Password... ");

    let mut user_password = String::new();

    io::stdin()
    .read_line(&mut user_password)
    .expect("Failed to read line");    

    let new_member = User::new_user(user_name.trim().to_string(), user_password.trim().to_string());

    // new_member.print_user();
    if  user_path {
        print!("Attempting to log you in ...");
    } else {
        create_user_file(&new_member.username, &new_member.password).unwrap();
    }
    // 
    new_member.login();
}
