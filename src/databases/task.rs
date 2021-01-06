use std::fs::{OpenOptions};
use std::fs;
use std::io::{Result};
use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::task::Task;
use crate::models::user::LoginInfo;
use crate::constants;

pub fn add_task(content: &str) -> Result<()>{
    let current_user = get_current_user();
    if current_user.is_none() {
        println!("Please login.");
        return Ok(());
    }

    let user_id = current_user.unwrap().id;

    let path = file_path();
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path);

    match get_metadata(path) {
        Ok(mut tasks) => {
            let id = tasks.len()  as i32 + constants::TASK_ID_INCREMENT_THRESHOLD;
            let task = Task::new(content, id, user_id);
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
    let current_user = get_current_user();
    if current_user.is_none() {
        println!("Please login.");
        return Ok(());
    }

    let user_id = current_user.unwrap().id;

    let path = file_path();

    match get_metadata(path) {
        Ok(mut tasks) => {
            match tasks.iter_mut().find(|task| task.id == id && user_id == user_id) {
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
    let current_user = get_current_user();
    if current_user.is_none() {
        println!("Please login.");
        return Ok(());
    }

    let user_id = current_user.unwrap().id;

    let path = file_path();

    match get_metadata(path) {
        Ok(tasks) => {
            let finished_tasks: Vec<Task> = tasks
                .iter()
                .filter(|task| task.finished && task.user_id == user_id)
                .cloned()
                .collect();
            let unfinished_tasks: Vec<Task> = tasks
                .iter()
                .filter(|task| !task.finished && task.user_id == user_id)
                .cloned()
                .collect();
            let finished_count = finished_tasks.len();
            let total = finished_count + unfinished_tasks.len();

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
    let current_user = get_current_user();
    if current_user.is_none() {
        println!("Please login.");
        return Ok(());
    }
    let user_id = current_user.unwrap().id;

    let path = file_path();

    match get_metadata(path) {
        Ok(tasks) => {
            let unfinished_tasks: Vec<Task> = tasks
                .iter()
                .filter(|task| !task.finished && task.user_id == user_id)
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

pub fn export_tasks(file_name: &str) -> Result<()> {
    let current_user = get_current_user();
    if current_user.is_none() {
        println!("Please login.");
        return Ok(());
    }

    if fs::metadata(constants::TASKS_FILE).is_err() {
        println!("There is not task");
        return Ok(())
    }

    if fs::metadata(constants::DOWNLOAD_DIR).is_err() {
        fs::create_dir(constants::DOWNLOAD_DIR);
    }

    let download_file_path = &format!("{download_dir}/{file_name}.json",
                                     download_dir = constants::DOWNLOAD_DIR,
                                     file_name = file_name);
    let download_path = Path::new(download_file_path);
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&download_path);

    let user_id = current_user.unwrap().id;
    let original_path = file_path();
    match get_metadata(original_path) {
        Ok(tasks) => {
            let download_tasks: Vec<Task> = tasks
                .iter()
                .filter(|task| task.user_id == user_id)
                .cloned()
                .collect();

            let json: String = serde_json::to_string(&download_tasks)?;
            fs::write(&download_path, &json).expect("Unable write to file");

            let total = download_tasks.len();
            let word = get_singular_plural(total, "item".to_string());
            println!("Export success. {total} {word} exported.", total = total, word = word);
        },
        Err(_) => {
            println!("get json metadata wrong.");
        }
    }

    Ok(())
}

pub fn import_tasks(file_name: &str) -> Result<()> {
    let current_user = get_current_user();
    if current_user.is_none() {
        println!("Please login.");
        return Ok(());
    }

    if !file_name.ends_with(".json") {
        println!("File must ends with .json");
        return Ok(());
    }
    if fs::metadata(file_name).is_err() {
        println!("File does not exist");
        return Ok(());
    }

    let waiting_task_path = Path::new(file_name);
    match get_metadata(waiting_task_path) {
        Ok(waiting_tasks) => {
            let task_file_path = &file_path();
            if fs::metadata(constants::TASKS_FILE).is_err() {
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(&task_file_path);
            }

            let mut tasks = get_metadata(task_file_path).unwrap();
            let mut success_count = 0;
            let mut fail_count = 0;
            for waiting_task in waiting_tasks.iter() {
                // 如果两条 task 的 id, user_id, content 同时相同，即认为是重复数据，不执行导入
                if tasks
                    .iter()
                    .any(|task| task.id == waiting_task.id && task.user_id == waiting_task.user_id && task.content == waiting_task.content) {
                    fail_count += 1;
                    continue;
                }
                success_count += 1;
                tasks.push(waiting_task.clone());
            }

            let json: String = serde_json::to_string(&tasks)?;
            fs::write(&task_file_path, &json).expect("Unable write to file");

            println!("Import success, success {}, failed {}", success_count, fail_count);
        },
        Err(_) => {
            println!("get json metadata wrong.");
        }
    }


    Ok(())
}

fn file_path() -> &'static Path {
    Path::new(constants::TASKS_FILE)
}

fn get_metadata(path: &Path) -> Result<Vec<Task>> {
    let mut tasks: Vec<Task> = vec![];
    if fs::metadata(&path).is_err() {
        return Ok(tasks);
    }

    let string_data = fs::read_to_string(&path).expect("Unable to read file");
    if fs::metadata(&path).unwrap().len() != 0 {
        tasks = serde_json::from_str(&string_data)?;
    }

    Ok(tasks)
}

fn get_singular_plural(count: usize, word: String) -> String {
    if count > constants::SINGULAR_PLURAL_THRESHOLD as usize {
        format!("{}s", word)
    } else {
        format!("{}", word)
    }
}

fn get_current_user() -> Option<LoginInfo> {
    let cache_path = Path::new(constants::CACHE_FILE);
    if fs::metadata(constants::CACHE_FILE).is_err() {
        return None;
    }
    let string_data = fs::read_to_string(&cache_path).expect("Unable to read file");
    let mut login_info = LoginInfo::new();
    if fs::metadata(&cache_path).unwrap().len() != 0 {
        login_info = serde_json::from_str(&string_data).expect("Unable get json data");
    }

    // 用户退出登录，恢复 LoginInfo 结构为初始状态，此时 id 值为 0
    if login_info.id == 0 {
        return None;
    }

    Some(login_info)
}
