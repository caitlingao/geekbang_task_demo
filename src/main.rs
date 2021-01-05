use std::{env, fs};
use std::io::{stdin, stdout, Write};

use databases::{task, user};
use constants::TMP_DIR;

mod constants;
mod databases;
mod models;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 2 {
        println!("error command");
        return;
    }

    // 预先创建 tmp 目录，以便于存储用户登录 cache 数据与 task 数据；
    if fs::metadata(TMP_DIR).is_err() {
        fs::create_dir(TMP_DIR);
    }

    let cmd = &args[1];

    match &cmd[..] {
        "login" => {
            if args.get(2).is_none() {
                println!("subcommand can not empty.");
                return;
            }

            if args[2] != "-u" {
                println!("Please use -u");
                return;
            }

            if args.get(3).is_none() {
                println!("email can not empty.");
                return;
            }

            println!("Password:");
            stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let password = input.trim();
            user::login(&args[3], password);
        },
        "logout" => {
            user::logout();
        },
        "add" => {
            if args.get(2).is_none() {
                println!("content can not empty.");
                return;
            }
            task::add_task(&args[2]);
        },
        "done" => {
            if args.get(2).is_none() {
                println!("item id can not empty.");
                return;
            }
            match &args[2].parse::<i32>() {
                Ok(n) => {
                    task::finish_task(*n);
                },
                Err(_) => {
                    println!("invalid index, item index must i32");
                    return;
                }
            }
        },
        "list" => {
            if args.get(2).is_none() {
                task::get_unfinished_tasks();
            } else {
                let sub_cmd = &args[2];
                match &sub_cmd[..] {
                    "--all" => {
                        task::get_tasks();
                    },
                    _ => {
                        println!("invalid command, sub command should be --all");
                        return;
                    }
                }
            }
        }
        _ => {
            println!("invalid command, command should be login | logout | add | done | list");
        }
    }
}
