mod task;
mod in_memory_task_repo;
mod task_repo;

use chrono::Utc;
use in_memory_task_repo::InMemoryTaskRepo;
use task_repo::TaskRepo;
use task::Task;

fn main() {
    let task = Task::new("Do homework".to_string(), None, false);
    println!("Task: {task:?}");

    let mut repo = InMemoryTaskRepo::new();
    repo.add(task);
    repo.add(Task::new("Prepare dinner".to_string(), Some(Utc::today().naive_local()), false));

    println!("Tasks in repo: {:#?}", repo.list());
    println!("Tasks in repo: {repo}");
}
