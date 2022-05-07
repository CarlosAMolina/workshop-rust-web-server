fn main() {
    let mut buffer = format!("Hello");
    let slice = &buffer; // buffer borrowed here
    //buffer.push_str(" World"); // ERROR. Cannot mutate while shared
    //slice.push_str(" World"); // ERROR. Cannot mutate through a shared ref
    println!("{:?}", slice); // reading slice ok while shared
    buffer.push_str(" World"); // after last use of slice, buffer is mutable again
}
