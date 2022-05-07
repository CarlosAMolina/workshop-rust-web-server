fn main() {
    let mut tribe_name = String::from("Backend"); // Mutable
    println!("Hi {}!", tribe_name); // Hi Backend!
    tribe_name.push_str(" Tribe");
    println!("Hi {}!", tribe_name); // Hi Backend Tribe!
    greet_borrowing(&tribe_name); // Hi Backend Tribe!
    println!("Hi {}!", tribe_name); // Hi Backend Tribe!
    greet_ownership(tribe_name); // Hi Backend Tribe!
    //println!("Hello {} Tribe!", tribe_name); // ERROR
}

fn greet_borrowing(name: &String) {
    println!("Hi {}!", name);
}

fn greet_ownership(name: String) {
    println!("Hi {}!", name);
}
