mod task;

use task::Task;

fn main() {
    let task = Task("Do homework".to_string(), false);
    println!("Task: {task:?}");
}
