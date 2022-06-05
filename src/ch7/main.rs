use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    // package: rust-notes
    // crate: ch7
    // binary crate (since main function exists)
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}