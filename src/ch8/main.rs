use std::collections::HashMap;
use std::ops::{Add, AddAssign};

fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    {
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
    } // v is returned when scope is closed
    println!("{:#?}", v);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100]; // panic
    let does_not_exist = v.get(100); // None
    println!("{:?}", does_not_exist);

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // immutable reference
    v.push(6); // mutable borrow
    // println!("The first element is: {}", first); // reference to immutable throws error
    // this behavior may be different depending on how the data structure is implemented

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i *= 2;
        println!("{}", i);
    }

    // use enums to store multiple "types" in a vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let mut s = String::new();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    let mut s = String::from("foo");
    let s2 = String::from("bar");
    s.push_str(&s2); // takes ref as a parameter
    let s3 = "!";
    s += &s3;
    // s += s2; // takes ref as parameter
    println!("s2: {}, s: {}, s3: {}", s2, s, s3);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // println!("s1: {}", s1); // can't use s1 as it ownership was transferred
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + &s2 + &s3;

    let s1 = String::from("tic");
    let s = format!("{}-{}-{}", s1, s2, s3); // use format since concatenation can be verbose
    println!("{}", s);

    let s1 = String::from("hello");
    // let h = s1[0]; // indexing not supported for String

    let hello = String::from("Hola");
    println!("{}", hello.len());
    let hello = String::from("Здравствуйте");
    println!("{}", hello.len()); // returns the number of bytes
    // let h = hello[0]; // string is a wrapper for u8 vector, so indexing isn't supported

    let hello = "Здравствуйте";
    // iterate over bytes
    // let s = &hello[0..1]; // panic
    let s = &hello[0..4]; // first 4 bytes
    println!("{}", s);
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // iterate over characters
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // iterating over grapheme aren't available from the standard library
    // https://crates.io/crates/unicode-segmentation

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{} : {}", field_name, field_value); // ownership of these values have been transferred

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(v) => {
            println!("found {}", v);
        }
        None => {
            println!("not found");
        }
    };
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    println!("{:?}", scores.get(&team_name));
    scores.insert(String::from("Blue"), 100);
    println!("{:?}", scores.get(&team_name));
    scores.entry(String::from("Red")).or_insert(100); // inserted with 100
    scores.entry(String::from("Blue")).or_insert(50); // ignored since key already exists
    println!("{:?}", scores);
    let mut res = scores.entry(String::from("Green")).or_insert(0);
    // println!("{:?}", &scores); // holds a mutable reference to scores
    *res += 1;
    println!("{:?}", &scores); // now we can print it
}