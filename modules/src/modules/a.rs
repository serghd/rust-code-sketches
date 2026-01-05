pub fn a_print() {
    println!("a_print");
}

#[derive(Debug)]
pub struct S {
    pub name: String,
    pub id: u32,
}

use crate::modules::foo;

pub fn call_foo_from_a() {
    foo::foo_print();
}
