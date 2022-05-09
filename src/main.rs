fn main() {
    let mut whom: &str = "World";
    println!("Hello, {whom}!");
    whom = "Planet";
    println!("Bye, {whom}!");

    // Allocated in the stack
    let x = String::from("This is not a &str");
    use_string(&x);
    println!("x = {x}");

    let y = "5";
    let y: Option<i32> = y.parse().ok();
    match y {
        Some(v) => {
            println!("y = {}", v);
        }
        None => {
            println!("ERR: Couldn't convert.");
        }
    }
}

fn use_string(s: &str) {
    println!("Inside use_string(): s = {s}");
}
