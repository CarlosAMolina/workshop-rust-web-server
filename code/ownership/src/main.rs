fn main() {
    let tribe_name = String::from("Backend");
    println!("Hello {} tribe!", tribe_name);
    greet_tribe(tribe_name);
    // println!("Hello {} tribe!", tribe_name); // ERROR
}

fn greet_tribe(name: String) {
    println!("Hello {} tribe!", name);
}

