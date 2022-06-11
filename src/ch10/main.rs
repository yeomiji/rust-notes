use std::fmt::{Debug, Display, Formatter};

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // println!("distance: {}", integer.distance_from_origin()); // not defined
    println!("distance: {}", float.distance_from_origin());
    // let wont_work = Point { x: 5, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };

    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3: {:?}", p3);

    // no performance impact for using generics
    // rust creates type specific code at compile-time
    let integer = Some(5);
    let float = Some(5.0);
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);

    // Note: can only implement a trait for a impl if either the trait or type is local to the crate
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());
    notify(&article);

    let p = Pair {x: 12, y: 3};
    p.cmp_display();

    let s = 3.to_string();
    let ts = TestStruct {
        x: 3,
    };
    println!("{}", ts.to_string());

    {
        let r;
        {
            let x = 5;
            r = &x;
        }
        // println!("r: {}", r); // exception is thrown since &x is returned
        // this is rejected by the compiler since:
        // 1) the compiler checks the lifetime of r (a')
        // 2) the compiler checks the lifetime of x which is assigned to r (b')
        // 3) the compiler compares the lifetime, and rejects since b' is shorter than a'
    }
    {
        let r;
        let x = 5;
        r = &x;
        println!("{}", r) // lifetime end 11111111111111` r
    } // lifetime end of x
    // compiles since lifetime of r is shorter than liftime of x

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // println!("The longest string is {}", result); // a' lives up to string2 lifecycle

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest2(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result); // string 1 only is annotated

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // static lifetime, which lives for the program lifetime
    let s: &'static str = "I have a static lifetime.";
}

// Summary
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 1. the compiler adds lifetime references to all method parameters
//     e.g. fn foo<'a, 'b>(x: &'a i32, y: &'b i32);
// 2. if there is one input lifetime reference, this reference is applied to all output parameters
//     e.g. fn foo<'a>(x: &'a i32) -> &'a i32
// 3. if self exists as one of the method input parameters, it is applied to all output parameters
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

impl<'a> ImportantExcerpt<'a> {
    // rule 1,3 applied
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// the struct can't outlive part
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// it's not clear which reference will be used, so the compiler can tell
// the lifetime of each reference passed in.
// 'a marks the lifetime to be the shorter of x and y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    // y // doesn't compile since not lifetime annotated
    // let result = String::from("really long string");
    // result.as_str() // not allowed to return value not lifetime annotated
    x
}

struct TestStruct {
    x: i32,
}

impl Display for TestStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.x)
    }
}
// Any type that implements Display can use to_string (Blanket implementation)
// impl<T: Display> ToString for T {
//     fn to_string(&self) -> String {
//         todo!()
//     }
// }

// traits conditionally implement methods depending on trait bounds
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// can't return different types of traits
// fn returns_summarizable_multiple(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

pub fn notify(item: &impl Summary) { // accepts an implementation of Summary
    println!("Breaking news! {}", item.summarize());
}

// equivalent to notify, but more verbose
pub fn notify2<T: Summary>(item: &T) { // accepts a type which implements Summary
    println!("Breaking news! {}", item.summarize());
}

// item1 and item2 can have different types
pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
}

// item1 and item2 must have the same type
pub fn notify4<T: Summary>(item1: &T, item2: &T) {
}

// multiple trait bounds
pub fn notify5(item: &(impl Summary + Display)) {}
pub fn notify6<T: Summary + Display>(item: &T) {}
// where syntax for multiple trait bounds
pub fn notify7<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}
pub fn notify8<T, U>(t: &T, u: &U) where T: Display + Clone,
                                         U: Clone + Debug {}

pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author()) // default impl
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    // fn summarize(&self) -> String {
    //     self.summarize() // can't call default impl from overriden impl
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

#[derive(Debug)]
struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    // generics for methods
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

// also works for enums
enum Result<T, E> {
    Ok(T),
    Err(E),
}


fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
