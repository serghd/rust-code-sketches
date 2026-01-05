use crate::modules::foo;

pub fn call_foo() {
    foo::foo_print();
}

pub fn b_print() {
    println!("b_print");
}