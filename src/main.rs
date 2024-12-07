use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct TodoItem {
    id: usize,
    description: String,
    completed: bool,
}

fn load_todos(file_path: &str) -> HashMap<usize, TodoItem> {
    match fs::read_to_string(file_path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => HashMap::new(),
    }
}

fn save_todos(file_path: &str, todos: &HashMap<usize, TodoItem>) {
    let content = serde_json::to_string_pretty(todos).expect("Failed to serialize todos");
    fs::write(file_path, content).expect("Failed to write to file");
}


fn add_todo(todos: &mut HashMap<usize, TodoItem>, description: String) {
    let id = todos.len() + 1;
    let todo = TodoItem {
        id,
        description,
        completed: false,
    };
    todos.insert(id, todo);
    println!("Task added!");
}

fn delete_todo(todos: &mut HashMap<usize, TodoItem>, id: usize) {
    if todos.remove(&id).is_some() {
        println!("Task {} deleted!", id);
    } else {
        println!("Task {} not found!", id);
    }
}

fn mark_done(todos: &mut HashMap<usize, TodoItem>, id: usize) {
    if let Some(task) = todos.get_mut(&id) {
        if task.completed {
            println!("Task {} is already marked as done!", id);
        } else {
            task.completed = true;
            println!("Task {} marked as done!", id);
        }
    } else {
        println!("Task {} not found!", id);
    }
}


fn main() {
    let file_path = "todos.json";
    let mut todos = load_todos(file_path);

    println!("Welcome to TODO App!");
    println!("1. Add Task\n2. View Tasks\n3. Delete Task\n4. Mark Task as Done\n5. Quit");

    loop {
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                print!("Enter task description: ");
                io::stdout().flush().unwrap();

                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();

                add_todo(&mut todos, description.trim().to_string());
                save_todos(file_path, &todos);
            }
            "2" => {
                println!("Your tasks:");
                for (_, task) in &todos {
                    println!(
                        "{}. {} [{}]",
                        task.id,
                        task.description,
                        if task.completed { "Done" } else { "Not Done" }
                    );
                }
            }
            "3" => {
                print!("Enter task ID to delete: ");
                io::stdout().flush().unwrap();

                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();

                if let Ok(id) = id.trim().parse::<usize>() {
                    delete_todo(&mut todos, id);
                    save_todos(file_path, &todos);
                } else {
                    println!("Invalid ID!");
                }
            }
            "4" => {
                print!("Enter task ID to done: ");
                io::stdout().flush().unwrap();

                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();

                if let Ok(id) = id.trim().parse::<usize>() {
                    mark_done(&mut todos, id);
                    save_todos(file_path, &todos);
                } else {
                    println!("Invalid ID!");
                }
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice, try again.");
            }
        }
    }
}