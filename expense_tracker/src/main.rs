use serde::{Deserialize, Serialize};
use std::fmt;
use std::{
    fs,
    io::{self, Write},
};

#[derive(Serialize, Deserialize)]
struct Expense {
    id: u32,
    amount: f64,
    category: String,
}

impl Expense {
    fn new(id: u32, amount: f64, category: String) -> Self {
        Self {
            id,
            amount,
            category,
        }
    }
}

impl fmt::Display for Expense {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: {} | Category: {} | Amount: {}",
            self.id, self.category, self.amount
        )
    }
}

const FILE_PATH: &str = "expense.json";

fn save_task_in_json(expenses: &Vec<Expense>) {
    let json = serde_json::to_string_pretty(expenses).expect("Failed to convert tasks to JSON");

    fs::write(FILE_PATH, json).expect("Failed to write file");
}
fn load_all_expense() -> Vec<Expense> {
    let data = fs::read_to_string(FILE_PATH);

    match data {
        Ok(content) => serde_json::from_str(&content).unwrap_or(Vec::new()),

        Err(_) => Vec::new(),
    }
}

fn add_expense(expenses: &mut Vec<Expense>) {
    let mut input_amount = String::new();
    let mut category = String::new();

    println!("Enter your amount: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input_amount)
        .expect("Failed to read amount");

    let amount: f64 = input_amount
        .trim()
        .parse()
        .expect("Please enter a valid integer");

    println!("Enter your category: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut category)
        .expect("Failed to read category");

    let id = if expenses.is_empty() {
        0
    } else {
        expenses.last().unwrap().id + 1
    };
    let expense = Expense::new(id, amount, category.trim().to_string());
    expenses.push(expense);

    println!("Your Expense has been saved successfully");
}

fn show_expense(expenses: &Vec<Expense>) {
    if expenses.is_empty() {
        println!("No expenses found");
        return;
    }

    println!("\nAll Expenses:\n");

    for expense in expenses {
        println!("{}", expense);
    }
}

fn total_expense(expenses: &Vec<Expense>) {
    if expenses.is_empty() {
        println!("No expenses found");
        return;
    }
    let total_sum: f64 = expenses.iter().map(|expense| expense.amount).sum();

    println!("Total Expense: {}", total_sum);
}
fn main() {
    let mut expenses = load_all_expense();

    loop {
        println!("\n===== Expense Tracker =====");
        println!("1. Add Expense");
        println!("2. Show Expenses");
        println!("3. Total Expense");
        println!("4. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read choice");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match choice {
            1 => {
                add_expense(&mut expenses);
                save_task_in_json(&expenses);
            }

            2 => {
                show_expense(&expenses);
            }

            3 => {
                total_expense(&expenses);
            }

            4 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
}
