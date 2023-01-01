fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let (x, y, z) = (1, 2, 3);
    // let (x, y, z) = (1, 2, 3, 4); // won't compile

    let point = (3, 5);
    print_coordinates(&point);

    // let Some(x) = Some(3); // won't compile since refutable
    if let Some(x) = Some(3) {
        println!("{}", x);
    }
    if let x = 5 {
        println!("{}", x);
    }

    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point {x: 0, y: 7};
    let Point {x: a, y: b} = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    match p {
        Point {x, y: 0} => println!("On the x axis at {x}"),
        Point {x: 0, y} => println!("On the y axis at {y}"),
        Point {x, y} => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}");
            }
            Message::Write(text) => {
                println!("Text message: {text}");
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {r}, green {g}, and blue {b}", );
            },
        }

        {
            enum Color {
                Rgb(i32, i32, i32),
                Hsv(i32, i32, i32),
            }

            enum Message {
                Quit,
                Move { x: i32, y: i32 },
                Write(String),
                ChangeColor(Color),
            }

            let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
            match msg {
                Message::ChangeColor(Color::Rgb(r, g, b)) => {
                    println!("Change color to red {r}, green {g}, and blue {b}");
                },
                Message::ChangeColor(Color::Hsv(h, s, v)) => {
                    println!("Change color to hue {h}, saturation {s}, value {v}");
                },
                _ => (),
            }
        }

        let ((feet, inches), Point { x, y }) = ((3, 10), Point {x: 3, y: -10});
        fn foo(_: i32, y: i32) {
            println!("This code only uses the y parameter: {}", y);
        }
        foo(3, 4);

        let mut setting_value = Some(5);
        let new_setting_value = Some(10);
        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }
        println!("setting is {:?}", setting_value);

        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, _, third, _, fifth) => {
                print!("Some numbers: {first}, {third}, {fifth}");
            }
        }

        let _x = 5;
        let y = 10;

        let s = Some(String::from("Hello!"));
        // if let Some(_s) = s { // ownership is transferred
        if let Some(_) = s {
            println!("found a string");
        }
        println!("{:?}", s);

        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }
        let origin = Point {x: 0, y: 0, z: 0};
        match origin {
            Point {x, ..} => println!("x is {}", x);
        }

        match numbers {
            // (.., second, ..) => { // won't compile since ambiguous
            (first, .., last) => {
                println!("Some numbers: {first}, {last}");
            }
        }

        let num = Some(4);
        match num {
            // cannot check for exhaustiveness
            Some(x) if x % 2 == 0 => println!("The number {} is even", x),
            Some(x) => println!("The number {} is odd", x),
            None => (),
        }

        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched, n = {n}"),
            _ => println!("Default case, x = {:?}", x),
        }
        println!("at the end, x = {:?}, y = {y}", x)

        let x = 4;
        let y = false;
        match x {
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }

        {
            enum Message {
                Hello {id: i32},
            }
            let msg = Message::Hello {id: 5};
            match msg {
                Message::Hello { id: id_variable @ 3..=7 } => {
                    println!("Found an id in range: {}", id_variable);
                },
                Message::Hello { id: 10..=12 } => {
                    println!("Found an id in another range");
                },
                Message::Hello { id } => println!("Found some other id: {}", id),
            }
        }
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
