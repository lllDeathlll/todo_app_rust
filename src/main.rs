use std::env;
use std::fs::*;
use std::io;
use std::process::exit;
pub use todo_app_rust::{add_task, complete_task, delete_task, list_tasks};
fn main() {
    println!("Welcome to todo app");
    println!("You can add/complete/delete tasks");
    // Gets all arguments
    let args: Vec<String> = env::args().collect();
    // Gets name argument
    let filename = match &args.get(1) {
        Some(path) => path,
        None => "tasks",
    };
    // Makes path out of filename
    let path = &format!("{}.json", filename);
    // Creates file if it doesn't exist
    match File::open(path) {
        Ok(_) => {}
        Err(_) => {
            File::create(path).expect("Failed to create file");
        }
    };
    //Starts program
    loop {
        println!("Menu");
        println!("0. List tasks");
        println!("1. Add task");
        println!("2. Complete task");
        println!("3. Delete task");
        println!("4. Exit");
        // Get menu number user input
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        // Converts trimmed and parsed menu number to integer
        let answer: i32 = match answer.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                match err.to_string().as_str() {
                    "invalid digit found in string" => println!("You must enter a number\n"),
                    _ => println!("{}\n", err),
                }
                continue;
            }
        };
        // Chooses what to do based on user input
        match answer {
            0 => list_tasks(path),
            1 => add_task(path),
            2 => complete_task(path),
            3 => delete_task(path),
            4 => exit(0),
            _ => println!("You need to choose number between 1 and 4\n"),
        }
    }
}
