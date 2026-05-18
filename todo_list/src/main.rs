use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

const FILE_PATH: &str = "tasks.json";

#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

impl Task {
    fn new(id: u32, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
        }
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to convert tasks to JSON");

    fs::write(FILE_PATH, json).expect("Failed to write file");
}

fn load_tasks() -> Vec<Task> {
    let data = fs::read_to_string(FILE_PATH);

    match data {
        Ok(content) => serde_json::from_str(&content).unwrap_or(Vec::new()),

        Err(_) => Vec::new(),
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    let mut title = String::new();

    print!("Enter your task: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read input");

    let id = if tasks.is_empty() {
        1
    } else {
        tasks.last().unwrap().id + 1
    };

    let task = Task::new(id, title.trim().to_string());

    tasks.push(task);
    save_tasks(tasks);
    println!("Your Task has been added successfully");
}

fn print_task(task: &Task) {
    println!(
        "ID: {} | Task: {} | Completed: {}",
        task.id, task.title, task.completed
    );
}
fn find_task(tasks: &Vec<Task>) {
    let mut input = String::new();
    print!("Enter task ID to Search: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let id: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid ID");
            return;
        }
    };
    let mut low = 0;
    let mut high = tasks.len().saturating_sub(1);

    while low <= high {
        let mid = (low + high) / 2;

        if tasks[mid].id == id {
            print_task(&tasks[mid]);
            return;
        } else if tasks[mid].id > id {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    println!("Task not found");
}

fn delete_task(tasks: &mut Vec<Task>) {
    let mut input = String::new();
    print!("Enter task ID to delete: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let id: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid ID");
            return;
        }
    };

    let original_len = tasks.len();

    tasks.retain(|task| task.id != id);

    if tasks.len() < original_len {
        save_tasks(tasks);
        println!("Task deleted successfully");
    } else {
        println!("Task not found");
    }
}

fn complete_task(tasks: &mut Vec<Task>) {
    let mut input = String::new();

    print!("Enter task ID to complete: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let id: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid ID");
            return;
        }
    };

    for task in tasks.iter_mut() {
        if task.id == id {
            task.completed = true;
            save_tasks(tasks);
            println!("Task marked as completed");

            return;
        }
    }

    println!("Task not found");
}

fn show_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No Tasks Found!");
        return;
    }
    println!("\nTodo List:");
    for task in tasks {
        println!(
            "ID: {} | Task: {} | Completed: {}",
            task.id, task.title, task.completed
        );
    }
}
fn main() {
    let mut tasks = load_tasks();
    loop {
        println!("\n==== TODO APP ====");
        println!("1. Add Task");
        println!("2. Show Tasks");
        println!("3. Find Task");
        println!("4. Complete Task");
        println!("5. Delete Task");
        println!("6. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => show_tasks(&tasks),
            "3" => find_task(&tasks),
            "4" => complete_task(&mut tasks),
            "5" => delete_task(&mut tasks),
            "6" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option"),
        }
    }
}
