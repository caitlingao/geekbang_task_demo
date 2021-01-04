use std::fs::{OpenOptions};
use std::fs;
use std::io::{Result};
use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::Task;

const FILE_NAME: &str = "tasks.json";
const SINGULAR_PLURAL_THRESHOLD: usize = 1;

pub fn add_task(content: &str) -> Result<()>{
    let path = file_path();
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path);

    match get_metadata(path) {
        Ok(mut tasks) => {
            let id = tasks.len()  as i32 + 1;
            let task = Task::new(content, id);
            tasks.push(task);

            let json: String = serde_json::to_string(&tasks)?;
            fs::write(&path, &json).expect("Unable write to file");
            println!("{id}. {content}", id = id, content = content);
            println!("Item {id} added", id = id);

        },
        Err(_) => {
            println!("get json metadata wrong.");
        }
    }

    Ok(())
}

pub fn finish_task(id: i32) -> Result<()>{
    let path = file_path();

    match get_metadata(path) {
        Ok(mut tasks) => {
            match tasks.iter_mut().find(|task| task.id == id) {
                Some(task) => {
                    task.finished = true;
                    task.updated_at = chrono::offset::Utc::now();
                }
                None => {
                    println!("task does not exist");
                    return Ok(());
                }
            }

            let json = serde_json::to_string_pretty(&tasks)?;
            fs::write(&path, &json).expect("Unable write to file");

            println!("Item {id} done.", id=id);
        },
        Err(_) => {
            println!("get json metadata wrong.");
        }
    }

    Ok(())
}

pub fn get_tasks() -> Result<()> {
    let path = file_path();

    match get_metadata(path) {
        Ok(tasks) => {
            let finished_tasks: Vec<Task> = tasks
                .iter()
                .filter(|task| task.finished)
                .cloned()
                .collect();
            let unfinished_tasks: Vec<Task> = tasks
                .iter()
                .filter(|task| !task.finished)
                .cloned()
                .collect();
            let total = tasks.len();
            let finished_count = finished_tasks.len();

            for task in unfinished_tasks.iter() {
                println!("{id}. {content}", id = task.id, content = task.content);
            }

            for task in finished_tasks.iter() {
                println!("{id}. [Done]{content}", id = task.id, content = task.content);
            }


            println!();
            let total_word = get_singular_plural(total, "item".to_string());
            let finished_count_word = get_singular_plural(finished_count, "item".to_string());
            println!("Total: {total} {total_word}, {finished_count} {finished_count_word} done",
                     total = total,
                     total_word = total_word,
                     finished_count = finished_count,
                     finished_count_word = finished_count_word
            );

        },
        Err(_) => {
            println!("get json metadata wrong.");
        }
    }

    Ok(())
}

pub fn get_unfinished_tasks() -> Result<()> {
    let path = file_path();

    match get_metadata(path) {
        Ok(tasks) => {
            let unfinished_tasks: Vec<Task> = tasks
                .iter()
                .filter(|task| !task.finished)
                .cloned()
                .collect();
            let total = unfinished_tasks.len();

            for task in unfinished_tasks.iter() {
                println!("{id}. {content}", id = task.id, content = task.content);
            }

            println!();
            let word = get_singular_plural(total, "item".to_string());
            println!("Total: {total} {word}", total = total, word = word);
        },
        Err(_) => {
            println!("get json metadata wrong.");
        }
    }

    Ok(())
}

fn file_path() -> &'static Path {
    Path::new(FILE_NAME)
}

fn get_metadata(path: &Path) -> Result<Vec<Task>> {
    let string_data = fs::read_to_string(&path).expect("Unable to read file");
    let mut tasks: Vec<Task> = vec![];
    if fs::metadata(&path).unwrap().len() != 0 {
        tasks = serde_json::from_str(&string_data)?;
    }

    Ok(tasks)
}

fn get_singular_plural(count: usize, word: String) -> String {
    if count > SINGULAR_PLURAL_THRESHOLD {
        format!("{}s", word)
    } else {
        format!("{}", word)
    }
}
