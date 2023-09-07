use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

// Makes struct for tasks with two fields: "name" and "completed" with Serialize, Deserialize and Clone traits
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    name: String,
    completed: bool,
}

pub fn list_tasks(path: &str) {
    // Reads serialized contents from file
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error while reading contents: {}", err);
            return;
        }
    };
    // Deserializes contents
    let deserialized_contents: Vec<Task> = match serde_json::from_str(contents.as_str()) {
        Ok(tasks) => tasks,
        Err(err) => {
            match err.to_string().as_str() {
                "EOF while parsing a value at line 1 column 0" => {
                    println!("There are no tasks yet. Try creasing one");
                    println!(" ");
                }
                _ => eprintln!("Error while deserializing contents: {}", err),
            }
            return;
        }
    };
    println!("List of tasks:");
    // Prints every task with its number and status
    for task in deserialized_contents.into_iter().enumerate() {
        let (number, task) = task;
        let task_status = match task.completed {
            true => "Completed",
            false => "Not completed",
        };
        println!("{}) {}: {}", number + 1, task.name, task_status);
    }
    // Makes user input to continue
    let _ = io::stdin().read_line(&mut String::new());
    // Makes new line
    println!(" ");
}

pub fn add_task(path: &str) {
    // Get task name user input
    println!("Enter new task: ");
    let mut task_name = String::new();
    io::stdin()
        .read_line(&mut task_name)
        .expect("Failed to read line");
    // Makes Task struct from task name
    let task = Task {
        name: task_name.trim().to_owned(),
        completed: false,
    };
    // Reads serialized contents from file
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error while reading contents: {}", err);
            return;
        }
    };
    // Deserializes contents
    let mut deserialized_contents: Vec<Task> = match serde_json::from_str(contents.as_str()) {
        Ok(tasks) => tasks,
        Err(err) => {
            match err.to_string().as_str() {
                // If there is no tasks, creates task from zero
                "EOF while parsing a value at line 1 column 0" => {
                    let tasks = vec![task];

                    let json = serde_json::to_string(&tasks).unwrap();

                    match fs::write(path, json) {
                        Ok(_) => {}
                        Err(err) => eprintln!("{}", err),
                    };
                    println!(" ");
                }
                _ => eprintln!("Error while deserializing contents: {}", err),
            }
            return;
        }
    };
    // Adds task to the vector
    deserialized_contents.push(task);
    // Serializes updated contents
    let updated_contents_json = serde_json::to_string(&deserialized_contents).unwrap();
    // Writes updated contents to file
    match fs::write(path, updated_contents_json) {
        Ok(_) => {}
        Err(err) => eprintln!("{}", err),
    };

    println!(" ");
}

pub fn complete_task(path: &str) {
    // Gets line number user input
    println!("Enter line number: ");
    let mut line_number = String::new();
    io::stdin()
        .read_line(&mut line_number)
        .expect("Failed to read line");
    // Converts trimmed line number to usize
    let line_number: usize = match line_number.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            match err.to_string().as_str() {
                "invalid digit found in string" => println!("You must enter a number\n"),
                _ => println!("{}\n", err),
            }
            return;
        }
    };
    // Reads serialized contents from file
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error while reading contents: {}", err);
            return;
        }
    };
    // Deserializes contents
    let deserialized_contents: Vec<Task> = match serde_json::from_str(contents.as_str()) {
        Ok(tasks) => tasks,
        Err(err) => {
            match err.to_string().as_str() {
                "EOF while parsing a value at line 1 column 0" => {
                    println!("There are no tasks yet. Try creasing one");
                    println!(" ");
                }
                _ => eprintln!("Error while deserializing contents: {}", err),
            }
            return;
        }
    };
    // Changes task status on a line chosen by user
    let updated_contents: Vec<Task> = deserialized_contents
        .iter()
        .enumerate()
        .map(|(i, task)| {
            if i == line_number - 1 {
                Task {
                    name: task.name.clone(),
                    completed: !task.completed,
                }
            } else {
                task.clone()
            }
        })
        .collect();
    // Serializes updated contents
    let updated_contents_json = serde_json::to_string(&updated_contents).unwrap();
    // Writes updated contents to file
    match fs::write(path, updated_contents_json) {
        Ok(_) => {}
        Err(err) => eprintln!("{}", err),
    };
    // Makes new line
    println!(" ");
}

pub fn delete_task(path: &str) {
    // Reads serialized contents from file
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error while reading contents: {}", err);
            return;
        }
    };
    // Deserializes contents
    let deserialized_contents: Vec<Task> = match serde_json::from_str(contents.as_str()) {
        Ok(tasks) => tasks,
        Err(err) => {
            match err.to_string().as_str() {
                "EOF while parsing a value at line 1 column 0" => {
                    println!("There are no tasks yet. Try creasing one");
                    println!(" ");
                }
                _ => eprintln!("Error while deserializing contents: {}", err),
            }
            return;
        }
    };
    println!("List of tasks:");
    // Prints every task with its number and status
    for task in deserialized_contents.clone().into_iter().enumerate() {
        let (number, task) = task;
        let task_status = match task.completed {
            true => "Completed",
            false => "Not completed",
        };
        println!("{}) {}: {}", number + 1, task.name, task_status);
    }
    // Gets line number user input
    println!("Enter line number: ");
    let mut line_number = String::new();
    io::stdin()
        .read_line(&mut line_number)
        .expect("Failed to read line");
    // Converts trimmed line number to usize
    let line_number: usize = match line_number.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            match err.to_string().as_str() {
                "invalid digit found in string" => println!("You must enter a number\n"),
                _ => println!("{}\n", err),
            }
            return;
        }
    };
    // Removes line chosen by user
    let updated_contents: Vec<Task> = deserialized_contents
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != line_number - 1)
        .map(|(_, task)| task.clone())
        .collect();
    // Serializes updated contents
    let updated_contents_json = serde_json::to_string(&updated_contents).unwrap();
    // Writes updated contents to file
    match fs::write(path, updated_contents_json) {
        Ok(_) => {}
        Err(err) => eprintln!("{}", err),
    };
    // Makes new line
    println!(" ");
}
