// src/bin/cli.rs

use clap::{Parser, Subcommand};
use tasks_lib::*;
use std::error::Error;

#[derive(Parser)]
#[command(name = "Todo")]
#[command(about = "A simple command-line to-do app")]
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

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let path = ".tasks.json";
    let mut tasks = load_tasks(path).unwrap_or_default();

    match cli.command {
        Commands::Add { description } => {
            add_task(&mut tasks, &description);
            println!("Task added.");
        }
        Commands::List => list_tasks(&tasks),
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

    save_tasks(&tasks, path)?;
    Ok(())
}
