use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::models::user::*;
use crate::constants;

pub fn login(email: &str, password: &str) -> Result<(), Box<dyn Error>>{
    let path = Path::new(constants::USER_FILE);

    let string_data = fs::read_to_string(&path).expect("Unable to read file");
    let mut users: Vec<User> = vec![];
    if fs::metadata(&path).unwrap().len() != 0 {
        users = serde_json::from_str(&string_data)?;
    }
    match users.iter().find(|user| user.email == email && user.password == password) {
        Some(user) => {
            let cache_path = Path::new(constants::CACHE_FILE);
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&cache_path);

            let string_data = fs::read_to_string(&cache_path).expect("Unable to read file");
            let mut login_info = LoginInfo::new();
            if fs::metadata(&cache_path).unwrap().len() != 0 {
                login_info = serde_json::from_str(&string_data)?;
            }
            login_info.email = email.to_string();
            login_info.id = user.clone().id;
            login_info.name = user.clone().name;
            let cache_json = serde_json::to_string(&login_info)?;

            fs::write(&cache_path, &cache_json).expect("Unable write to file");

            println!("login success.");
        },
        None => {
            println!("user does not exist.")
        }
    }

    Ok(())
}

pub fn logout() -> Result<(), Box<dyn Error>>{
    let cache_path = Path::new(constants::CACHE_FILE);
    if fs::metadata(&cache_path).is_err() {
        println!("logout success.");
        return Ok(());
    }

    let login_info = LoginInfo::new();
    let cache_json = serde_json::to_string(&login_info)?;

    fs::write(&cache_path, &cache_json).expect("Unable write to file");

    println!("logout success.");

    Ok(())
}
