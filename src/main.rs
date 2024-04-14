use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
fn main() {
    let user_serch = user_input();
    let content = match read_file() {
        Ok(v) => v,
        Err(e) => {
            println!("Error reading file\nError: {}", e);
            return;
        }
    };

    sercher(user_serch, content);
}

/// Read
fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("D:/test.txt")?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// check is the input from the user equle with somhing in the file.
fn sercher(user_serch: String, file: String) {
    assert_eq!(user_serch, file);
}
/// Aske the user for a input and give it back as string ´user_input´
fn user_input() -> String {
    let mut user_input: String = String::new();
    println!("please enter a serch: ");
    let _ = stdout().flush();
    stdin().read_line(&mut user_input).unwrap();
    user_input
}
