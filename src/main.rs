use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::collections::HashMap;


fn main() {
    let (user_serch,file_name) = user_input();
    let content: String = match read_file(file_name) {
        Ok(v) => v,
        Err(e) => {
            println!("Error reading file\nError: {}", e);
            return;
        }
    };
   

   
sercher(user_serch, content);
}

/// Read
fn read_file(file_name:String) -> Result<String, std::io::Error> {
    let mut file = File::open(file_name)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)

}

/// Aske the user for a input and give it back as string ´user_input´
fn user_input() -> (String, String) {
    let mut user_input: String = String::new();
    let mut file_name : String = String::new();
    println!("please enter a serch: ");
    let _ = stdout().flush();
    stdin().read_line(&mut user_input).unwrap();

    println!("please enter a File: ");
    let _ = stdout().flush();
    stdin().read_line(&mut file_name).unwrap();

    (String::from(user_input.trim()),String::from(file_name.trim()))
}

/// check is the input from the user equle with somhing in the file.
fn sercher(user_serch: String, conntent: String) -> u8 {
    let mut x: u32 = 0;
    let mut y: u8 = 0;
    let user_serch: &str = user_serch.as_str();
    let mut word_counts:HashMap<String, u32> = HashMap::new();
    for word in conntent.split_whitespace() {
        let count = word_counts.entry(word.to_string()).or_insert(0);
        *count += 1;
        if word.contains(user_serch) {
            y = y + 1;
        }
        x = x + 1;
    }
  
    println!("{}/{}",y, x);
   return y;
}