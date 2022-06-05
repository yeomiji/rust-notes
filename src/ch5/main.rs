fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.username = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{} {}", user2.email, user1.email); // can use email
    // println!("{} {}", user2.username, user1.username); // but username was transferred

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("rect1 is {:#?}", rect1);
    let scale = 2;
    let mut rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1); // no need to worry about formatting

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    rect1.height(1);
    dbg!(&rect1);
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect = Rectangle::square(3);
    dbg!(&rect);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // &self is used as a reference, otherwise ownership would be transferred
        self.width * self.height
    }

    fn height(&mut self, height: u32) { // &mut reference to modify
        self.height = height;
    }

    fn width(&self) -> bool { // overriding variable name with method is possible, but overloading not possible
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool { // again, immutable borrow of another rectangle
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle { // multiple impl declarations possible
    fn square(size: u32) -> Rectangle { // same as python - don't pass self for static methods
        Rectangle {
            width: size,
            height: size,
        }
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Need specified lifetimes
// struct User2 {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
