fn main() {
    let s = "hello";
    {
        let s = "hello";
    }
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    {
        let s = String::from("hello");
    } // s is freed after this point == drop(s);

    let s1 = String::from("hello");
    let s2 = s1; // ownership is transferred, so can't use s1 now

    let s1 = String::from("hello");
    let s2 = s1.clone(); // clone to explicitly create a deep copy
    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x; // ownership not transferred, the data type (i32) implements the Copy trait
    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); <- can't do this since ownership is transferred
    let x = 5;
    makes_copy(x);
    println!("{}", x);
    let x = String::from("hello");
    let x = takes_and_gives_back(x);
    println!("{}", x);

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("{}", len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    let mut s = String::from("hello");
    let r1 = &mut s;
    // s.push_str(", world!"); <- s cannot be used anymore
    r1.push_str(", world!");
    // let r2 = &mut s; <- multiple borrows
    println!("{}", r1);

    // using scopes is fine
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;
    println!("{}", r2);

    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // can't use a mutable reference while immutable references are held
    // println!("{}, {}, and {}", r1, r2, r3);
    println!("{}, {}", r1, r2); // can use before

    let r3 = &mut s; // since r1, r2 have been used
    println!("{}", r3);


    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let slice = &s[0..2];
    let slice = &s[..2];
    let slice = &s[3..len];
    let slice = &s[3..];

    let mut s = String::from("hello world");
    let hello = first_word(&s); // deref coercion
    let hello = first_word(&s[..]);
    // s.clear(); // can't do this since hello is an immutable reference to s
    println!("{}", hello);
    s.clear(); // but can do this here since hello has been used
    let s = "hello world!";
    let s = first_word(s); // since literals are already a slice
    let s = first_word(&s);
    let s = first_word(&s[..]);
    println!("{}", s);

    // also for arrays
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s <- not allowed to return a reference to an object which will be dropped
// }

fn change(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}

fn calculate_length(s: &String) -> usize {
    // s.push_str(", world"); <- cannot modify references
    s.len()
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
