fn main() {
    let tribe_name = "Back";
    println!("Hello {} tribe!", tribe_name);
    greet_tribe(tribe_name);
    println!("Hello {} tribe!", tribe_name);
}

fn greet_tribe(name: &str) {
    println!("Hello {} tribe!", name);
}

