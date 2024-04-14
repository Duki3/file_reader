use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::collections::HashMap;


fn main() {
    let user_serch = user_input();
    let content = match read_file() {
        Ok(v) => v,
        Err(e) => {
            println!("Error reading file\nError: {}", e);
            return;
        }
    };
    println!("{}", content);

    let x = sercher(user_serch, content);
    print!("{}", x);
}

/// Read
fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("D:/test.txt")?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Aske the user for a input and give it back as string ´user_input´
fn user_input() -> String {
    let mut user_input: String = String::new();
    println!("please enter a serch: ");
    let _ = stdout().flush();
    stdin().read_line(&mut user_input).unwrap();
    user_input
}

/// check is the input from the user equle with somhing in the file.
fn sercher(user_serch: String, conntent: String) -> (u8) {
    let mut x: u8 = 0;
    let mut word_counts:HashMap<String, u32> = HashMap::new();
    for word in conntent.split_whitespace() {
        let count = word_counts.entry(word.to_string()).or_insert(0);
        *count += 1;
        x = x + 1;
    }
   return x;
}
