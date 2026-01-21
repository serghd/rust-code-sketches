mod samples;

use crate::samples::destructuring::City;
use crate::samples::enums::process_numbers;
use crate::samples::generics::process_objects;
use crate::samples::option;
use crate::samples::tuples::{tuple_destructuring, tuple_print};

fn main() {
    // // #1. Create and display object using Option
    // option::create_and_display_object();

    // // #2. Tuple Operations
    // tuple_print();
    // tuple_destructuring();

    // #3. Enums
    //process_numbers();

    // // #4. Destructuring
    // let city = City::create("City New".to_string(), "City Old".to_string(), 5000, 1219);
    // city.process_city();

    // #5. Generics
    process_objects();

    /////////////
}
