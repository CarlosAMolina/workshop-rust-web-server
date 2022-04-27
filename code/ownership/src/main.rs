fn main() {
    let tribe_name = String::from("Backend");
    println!("Hi {} Tribe!", tribe_name);
    greet_borrowing(&tribe_name);
    println!("Hi {} Tribe!", tribe_name);
    greet_takes_ownership(tribe_name);
    //println!("Hello {} Tribe!", tribe_name); // ERROR
}

fn greet_borrowing(name: &String) {
    println!("Hi {} Tribe!", name);
}

fn greet_takes_ownership(name: String) {
    println!("Hi {} Tribe!", name);
}
