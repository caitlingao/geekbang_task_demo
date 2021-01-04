use std::env;

use models::Task;
use database::*;

mod database;
mod models;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 2 {
        println!("error command");
        return;
    }

    let cmd = &args[1];
    match &cmd[..] {
        "add" => {
            if args.get(2).is_none() {
                println!("content can not empty.");
                return;
            }
            add_task(&args[2]);
        },
        "done" => {
            if args.get(2).is_none() {
                println!("item id can not empty.");
                return;
            }
            match &args[2].parse::<i32>() {
                Ok(n) => {
                    finish_task(*n);
                },
                Err(_) => {
                    println!("invalid index, item index must i32");
                    return;
                }
            }
        },
        "list" => {
            if args.get(2).is_none() {
                get_unfinished_tasks();
            } else {
                let sub_cmd = &args[2];
                match &sub_cmd[..] {
                    "--all" => {
                        get_tasks();
                    },
                    _ => {
                        println!("invalid command, sub command should be --all");
                        return;
                    }
                }
            }
        }
        _ => {
            println!("invalid command, command should be add | done | list");
        }
    }
}
