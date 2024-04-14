use std::fs::{read_link, File};
use std::io::{stdin, stdout, Read, Write};
use std::fs::read_to_string;
fn main() {
    let user_serch = user_input();
    let file = read_file(); 
    println!("{}",file.unwrap());
    sercher(user_serch, file); 

}

/// Read
fn read_file() -> Result<String,std::io::Error> {
    let mut file = File::open("D:/test.txt")?;
    let mut contents: String = String::new(); 
    file.read_to_string(&mut contents)?;
    Ok(contents)
}


/// check is the input from the user equle with somhing in the file.
fn sercher(user_serch:String, file : String) {
   assert_eq!(user_serch,"hello");  

}
/// Aske the user for a input and give it back as string ´user_input´
fn user_input() -> String{
    let mut user_input : String = String::new(); 
    println!("please enter a serch: "); 
    let _ = stdout().flush(); 
    stdin().read_line(&mut user_input).unwrap();
    user_input
}