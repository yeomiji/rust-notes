pub use restaurant;
use restaurant::{eat_at_restaurant, sibling};

fn main() {
    // bin can only use lib
    // lib can only use sibling mods
    // mods can only use mods in child directories with the same name as the mod
    eat_at_restaurant();
    sibling::whatever();
}