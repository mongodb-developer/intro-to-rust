fn main() {
    let mut whom: &str = "World";
    println!("Hello, {whom}!");
    whom = "Planet";
    println!("Bye, {whom}!");

    // Allocated in the stack
    // let x = String::from("This is not a &str");
    let x = "This is not a &str".to_string();
    println!("x = {x}");
    //     let y = x;
    //     println!("x = {x}, y = {y}");
}
