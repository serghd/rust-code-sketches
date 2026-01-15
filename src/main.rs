mod option;
mod tuples;

use crate::tuples::{tuple_destructuring, tuple_print};

fn main() {
    // // #1. Create and display object using Option
    // option::create_and_display_object();

    // // #2. Tuple Operations
    // tuple_print();
    // tuple_destructuring();

    let my_number: u8 = 10;
    match my_number {
       number @ 10 => {
           println!("result: {}", number)
       },
        _ => {
            
        }
    };
    //println!("{}", my_number2);








}
