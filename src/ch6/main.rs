fn main() {
    let four = IpAddr::V4(0,0,0,0);
    let six = IpAddr::V6(String::from("asdf"));
    route(four);
    route(six);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None; // explicit type since the compiler can't tell
    // for languages with null (java/c/etc..), there is always a risk a value is null
    //   - annotations somewhat help with this, but they aren't conclusive
    // for rust, we can assume that non-option types are not null safely (checked by compiler)

    fn value_in_cents(coin: Coin) -> u8 {
        match coin { // cases need to be exhaustive (or it won't compile)
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?}", six, none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // catch all
        _ => reroll(), // if we don't want to use the value
    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // no need to handle boilerplate for catch-all
    if let Some(max) = config_max { // if config_max matches the pattern, use max to..
        println!("The maximum is configured to be {}", max);
    }

    // can also use else with if-let pattern
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        println!("not matched");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
fn route(ip_kind: IpAddr) {}
