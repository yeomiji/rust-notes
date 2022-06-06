use std::fs::File;
use std::{fs, io};
use std::error::Error;
use std::io::{ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };
    // println!("{:?}", f);

    // Also can use else
    // File::open("hello.txtx").unwrap_or_else(|e| {
    //    if (e.kind() == ErrorKind::NotFound) {
    //        // do something
    //    }
    //     panic!("Problem opening this file");
    // });

    // let f = File::open("hello.txt").unwrap(); // return if returned, panic if err
    // let f = File::open("hello.txt")
    //     .expect("Failed to open hello.txt"); // equivalent to unwrap, but also adds a message

    // let f = File::open("hello.txt")?; // can only use ? on return types that extend FromResidual

    let f = File::open("hello.txt")?;
    Ok(())
}

// can also use ? with option
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// propagates the error up
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")

    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    // let mut f = File::open("hello.txt")?; // <- if ok continue with f, otherwise return e
    // let mut s = String::new();
    // f.read_to_string(&mut s)?; // if ok continue with f, otherwise return e
    // Ok(s)

    // let f = File::open("hello.txt");
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
}
