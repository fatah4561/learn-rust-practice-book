use core::panic;
use std::fs::File;
use std::fs;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    let greeting_file_result = File::open("greeting.txt"); // file not exist
    println!("result is `{}`", greeting_file_result.as_ref().err().unwrap());

    let greeting_fle = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("greeting.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // another more concise ex:
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    // unwrap success or panic
    let greeting_file = File::open("hello.txt").unwrap();

    // expect success or panic but the panic message can be customized
    let greeting_file = File::open("hello.txt")
    .expect("hello.txt should be included in this project");

    let username = match read_username_from_file() {
        Ok(username) => username,
        Err(err) => panic!("Error: {err}"),
    };
}


// propagating error (returning function error to the caller)
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e), // since this is last expression we can to not explicitly use `return`
    }
}

// propagating error but using the ? operator
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// more shorter with chaining method
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// standard library way:
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
