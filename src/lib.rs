use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub completed: bool,
}

pub fn add_task(tasks: &mut Vec<Task>, description: &str) {
    let task = Task {
        id: Uuid::new_v4().to_string(),
        description: description.to_string(),
        completed: false,
    };
    tasks.push(task);
}

pub fn get_task_mut<'a>(id: &str, tasks: &'a mut Vec<Task>) -> Option<&'a mut Task> {
    tasks.iter_mut().find(|task| task.id == id)
}

pub fn mark_done(id: &str, tasks: &mut Vec<Task>) -> bool {
    if let Some(task) = get_task_mut(id, tasks) {
        task.completed = true;
        true
    } else {
        false
    }
}

pub fn list_tasks(tasks: &Vec<Task>) {
    for task in tasks {
        let mark = if task.completed { "[x]" } else { "[ ]" };
        println!("{} {} - {}", mark, task.id, task.description);
    }
}

pub fn remove_task(tasks: &mut Vec<Task>, id: &str) -> bool {
    let original = tasks.len();
    tasks.retain(|task| task.id != id);
    tasks.len() != original
}

pub fn clear_completed(tasks: &mut Vec<Task>) {
    tasks.retain(|task| !task.completed);
}

pub fn save_tasks(tasks: &Vec<Task>, path: &str) -> Result<(), std::io::Error> {
    let json = serde_json::to_string_pretty(tasks)?;
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_tasks(path: &str) -> Result<Vec<Task>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader)?;
    Ok(tasks)
}
