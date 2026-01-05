use modules_project::modules;

fn main() {
    
    modules::a::a_print();
    
    let s = modules::a::S {
        id: 1, 
        name: "S".to_string(),
    };
    println!("{:?}", s);

    modules::foo::foo_print();
    modules::b::call_foo();
}
