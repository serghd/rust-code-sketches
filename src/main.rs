mod enums;
mod option;
mod tuples;

use crate::enums::process_numbers;
use crate::tuples::{tuple_destructuring, tuple_print};

fn main() {
    // // #1. Create and display object using Option
    // option::create_and_display_object();

    // // #2. Tuple Operations
    // tuple_print();
    // tuple_destructuring();

    // #3. Enums
    process_numbers();

    /////////////
}
