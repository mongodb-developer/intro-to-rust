mod task;
mod in_memory_task_repo;
mod task_repo;

use chrono::Utc;
use in_memory_task_repo::InMemoryTaskRepo;
use mongodb::{options::{ClientOptions, ResolverConfig}, Client};
use std::{env, error::Error};
use task_repo::TaskRepo;
use task::Task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let task = Task::new("Do homework".to_string(), None, false);
    println!("Task: {task:?}");

    let mut repo = InMemoryTaskRepo::new();
    repo.add(task);
    repo.add(Task::new("Prepare dinner".to_string(), Some(Utc::today().naive_local()), false));

    println!("Tasks in repo: {:#?}", repo.list());
    println!("Tasks in repo: {repo}");

    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;
    let client = Client::with_options(options)?;
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
	println!("- {}", name);
    }
    Ok(())
}
