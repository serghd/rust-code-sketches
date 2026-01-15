#[derive(Debug)]
struct User {
    name: String,
}

pub fn create_and_display_object() {
    let user: Option<User> = Some(User {
        name: String::from("vasya"),
    });

    if let Some(u) = &user {
        println!("#1 {:?}", u);
    };

    match &user {
        Some(u) => {
            println!("#2 {:?}", u);
        }
        None => {
            println!("None");
        }
    }
}
