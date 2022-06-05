mod front_of_house;
pub mod sibling;

// valid only for this scope
// use up to parent module, so that locally defined functions don't have collision for libraries
use crate::front_of_house::hosting; // defined privately

// It's idiomatic to use full path for enums/structs though
use std::collections::HashMap;
use std::collections::HashMap as HashMap2; // can rename imports in case of collision
pub use std::collections::HashMap as HashMap3; // defined publicly

// use std::cmp::Ordering;
// use std::io;
use std::{self, cmp::Ordering, io}; // defined in one line, self means "std" in this context
use std::collections::*; // bring in everything recursively

mod customer {
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        // so this needs to assume this function is at the crate root level
        hosting::add_to_waitlist();
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // note: eat_at_restaurant is a sibling of front_of_house, hence no pub is needed

    // Absolute path
    hosting::add_to_waitlist();
    // Relative path
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries"); // private

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
