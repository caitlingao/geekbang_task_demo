use std::{env, fs};
use std::io::{stdin, stdout, Write};

use structopt::StructOpt;

use databases::{task, user};
use constants::TMP_DIR;

mod constants;
mod databases;
mod models;

fn main() {
    // 预先创建 tmp 目录，以便于存储 task 数据及登录缓存数据
    if fs::metadata(TMP_DIR).is_err() {
        fs::create_dir(TMP_DIR);
    }

    let conf = ApplicationArguments::from_args();
    if let Some(command) = conf.command {
        match command {
            Command::Login(arg) => {
                handle_user_login(arg);
            }
            Command::Logout(_) => {
                user::logout();
            }
            Command::Task(arg) => {
                handle_task_action(arg);
            }
        }
    }
}

fn handle_user_login(arg: Login) {
    println!("Password:");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let password = input.trim();

    user::login(&arg.user, password);
}

fn handle_task_action(arg: Task) {
    if arg.add.is_some() {
        task::add_task(&arg.add.unwrap());
        return;
    }
    if arg.done.is_some() {
        task::finish_task(arg.done.unwrap());
        return;
    }
    if arg.list {
        if arg.all {
            task::get_tasks();
            return;
        }
        task::get_unfinished_tasks();
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
