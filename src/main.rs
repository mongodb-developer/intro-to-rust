fn main() {
    let mut whom: &str = "World";
    println!("Hello, {whom}!");
    whom = "Planet";
    println!("Bye, {whom}!");

    // Allocated in the stack
    let x = String::from("This is not a &str");
    use_string(x);
    println!("x = {x}");
}

fn use_string(s: String) {
    println!("Inside use_string(): s = {s}");
}
