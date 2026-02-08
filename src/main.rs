mod samples;

use crate::samples::collections::{binary_heap_demo, make_survey, vec_deque_demo};
use crate::samples::destructuring::City;
use crate::samples::enums::process_numbers;
use crate::samples::generics::process_objects;
use crate::samples::option;
use crate::samples::traits::{evaluate_trait, evaluate_trait_bounds, evaluate_trait_from, print_objects_as_ref, print_string_as_bytes};
use crate::samples::tuples::{tuple_destructuring, tuple_print};

fn main() {
    // // #1. Option
    // option::create_and_display_object();
    // option::display_weather();

    // // #2. Tuple Operations
    // tuple_print();
    // tuple_destructuring();

    // #3. Enums
    //process_numbers();

    // // #4. Destructuring
    // let city = City::create("City New".to_string(), "City Old".to_string(), 5000, 1219);
    // city.process_city();

    // // #5. Generics
    // process_objects();

    // #6. Collections
    //make_survey();
    //binary_heap_demo();
    //vec_deque_demo();

    // #7. Traits
    // evaluate_trait();
    // evaluate_trait_bounds();
    // evaluate_trait_from();
    //print_string_as_bytes("abc");
    print_objects_as_ref();

    /////////////
}
