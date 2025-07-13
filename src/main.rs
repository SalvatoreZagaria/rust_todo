use uuid::Uuid;
use std::fs::File;
use clap::{Parser, Subcommand};
use std::io::{BufReader, Write};
use serde::{Serialize, Deserialize};


#[derive(Parser)]
#[command(name = "Todo")]
#[command(about = "A simple command-line to-do app", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    Add { description: String },
    List,
    Done { id: String },
    Remove { id: String },
    Clear,
}


#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: String,
    description: String,
    completed: bool
}


fn add_task(tasks: &mut Vec<Task>, description: &str) {
    let task = Task {
        id: Uuid::new_v4().to_string(),
        description: description.to_string(),
        completed: false,
    };
    tasks.push(task);
}


fn get_task_mut<'a>(id: &str, tasks: &'a mut Vec<Task>) -> Option<&'a mut Task> {
    tasks.iter_mut().find(|task| task.id == id)
}


fn mark_done(id: &str, tasks: &mut Vec<Task>) -> bool {
    if let Some(task) = get_task_mut(id, tasks) {
        task.completed = true;
        true
    } else {
        false
    }
}


fn list_tasks(tasks: &Vec<Task>) {
    for task in tasks {
        let line: String;
        if task.completed {
            line = String::from("[x]");
        } else {
            line = String::from("[ ]");
        }
        println!("{} {} - {}", line, task.id, task.description);
    }
}


fn remove_task(tasks: &mut Vec<Task>, id: &str) -> bool {
    let original_len = tasks.len();
    tasks.retain(|task| task.id != id);
    original_len != tasks.len()
}


fn clear_completed(tasks: &mut Vec<Task>) {
    tasks.retain(|task| !task.completed);
}


fn save_tasks(tasks: &Vec<Task>, path: &str) -> Result<(), std::io::Error> {
    let json = serde_json::to_string_pretty(tasks)?;
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}


fn load_tasks(path: &str) -> Result<Vec<Task>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader)?;
    Ok(tasks)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let tasks_file_str = String::from(".tasks.json");
    let mut tasks = load_tasks(&tasks_file_str).unwrap_or_else(|_| Vec::new());

    match cli.command {
        Commands::Add { description } => {
            add_task(&mut tasks, &description);
            println!("Task added.");
        }
        Commands::List => {
            list_tasks(&tasks);
        }
        Commands::Done { id } => {
            if mark_done(&id, &mut tasks) {
                println!("Task marked as done.");
            } else {
                println!("Task not found.");
            }
        }
        Commands::Remove { id } => {
            if remove_task(&mut tasks, &id) {
                println!("Task removed.");
            } else {
                println!("Task not found.");
            }
        }
        Commands::Clear => {
            clear_completed(&mut tasks);
            println!("Completed tasks cleared.");
        }
    }

    save_tasks(&tasks, &tasks_file_str)?;
    Ok(())
}
