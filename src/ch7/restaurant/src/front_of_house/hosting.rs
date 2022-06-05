// needs to be in subdirectory to be "imported" from other modules
// the only exception is the "crate root" (which in this case is lib)

use crate::front_of_house::everyone_can_access;
pub mod grandchild;

pub fn add_to_waitlist() {
    everyone_can_access();
}

fn seat_at_table() {}