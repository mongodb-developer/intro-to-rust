use std::fmt;

use crate::task::Task;
use crate::task_repo::TaskRepo;

#[derive(Debug)]
pub struct InMemoryTaskRepo {
    tasks: Vec<Task>,
}

impl InMemoryTaskRepo {
    pub fn new() -> InMemoryTaskRepo {
	InMemoryTaskRepo { tasks: vec![] }
    }
}

impl TaskRepo for InMemoryTaskRepo {
    fn add(&mut self, task: Task) {
	self.tasks.push(task);
    }

    fn list(&self) -> Vec<Task> {
	self.tasks.clone()
    }
}

impl fmt::Display for InMemoryTaskRepo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	write!(f, "Repo has {} tasks.", self.tasks.len())
    }
}
