#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::{env, fs};
use std::io::{stdin, stdout, Write};

use structopt::StructOpt;
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use services::*;
use constants::TMP_DIR;

mod config;
mod constants;
mod models;
mod schema;
mod services;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABaSe");
    let conn = PgConnection::establish(&database_url).unwrap();

    // 预先创建 tmp 目录，以便于存储 task 数据及登录缓存数据
    if fs::metadata(TMP_DIR).is_err() {
        fs::create_dir(TMP_DIR);
    }

    let conf = ApplicationArguments::from_args();
    if let Some(command) = conf.command {
        match command {
            Command::Login(arg) => {
                handle_user_login(arg, &conn);
            }
            Command::Logout(_) => {
                handle_user_logout();
            }
            Command::Task(arg) => {
                handle_task_action(arg, &conn);
            }
        }
    }
}

fn handle_user_login(arg: Login, conn: &PgConnection) {
    println!("Password:");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let password = input.trim();
    account_service::login(&arg.user, password, conn);
}

fn handle_user_logout() {
    account_service::logout();
}

fn handle_task_action(arg: Task, conn: &PgConnection) {
    let current_user = account_service::get_current_user();
    if current_user.is_none() {
        println!("{}", constants::ASK_FOR_LOGIN);
        return;
    }
    let user_id = current_user.unwrap().id;
    if arg.add.is_some() {
        task_service::insert_task(&arg.add.unwrap(), user_id, conn);
        return;
    }
    if arg.done.is_some() {
        task_service::finish_task(arg.done.unwrap(), user_id, conn);
        return;
    }
    if arg.list {
        if arg.all {
            task_service::find_all(user_id, conn);
            return;
        }
        task_service::find_unfinished(user_id, conn);
        return;
    }
    if arg.export {
        if arg.target.is_some() {
            task_service::export_tasks(user_id, &arg.target.unwrap(), conn);
            return;
        }
    }
    if arg.import {
        if arg.file.is_some() {
            task_service::import_tasks(&arg.file.unwrap(), conn);
            return;
        }
    }
    if arg.init {
        task_service::init_tasks(conn);
        account_service::import_users(conn);
        return;
    }
}

#[derive(Debug, StructOpt)]
pub struct Login {
    #[structopt(short, long)]
    pub user: String,
}

#[derive(Debug, StructOpt)]
pub struct Logout {
    #[structopt(short, long)]
    pub user: Option<String>,
}

#[derive(Debug, StructOpt)]
pub struct Task {
    #[structopt(long)]
    pub add: Option<String>,

    #[structopt(short, long)]
    pub done: Option<i32>,

    #[structopt(short, long)]
    pub list: bool,

    #[structopt(long)]
    pub all: bool,

    #[structopt(short, long)]
    pub export: bool,

    #[structopt(short, long)]
    pub target: Option<String>,

    #[structopt(short, long)]
    pub import: bool,

    #[structopt(short, long)]
    pub file: Option<String>,

    #[structopt(long)]
    pub init: bool,

}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "login")]
    Login(Login),
    #[structopt(name = "logout")]
    Logout(Logout),
    #[structopt(name = "todo")]
    Task(Task)
}

#[derive(Debug, StructOpt)]
#[structopt(name = "task")]
pub struct ApplicationArguments {
    #[structopt(subcommand)]
    pub command: Option<Command>,
}
