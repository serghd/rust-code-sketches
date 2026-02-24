pub fn call_thread() {
    let str = "My string".to_string();
    let handle = std::thread::spawn(move||{
        println!("{}", str);
    });
    handle.join().unwrap();
}