use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

const FILE_NAME: &str = "todo.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = load_tasks()?;

    loop {
        {
            println!("\n--- To-Do Menu ---");
            println!("1. List tasks");
            println!("2. Add task");
            println!("3. Remove task");
            println!("4. Quit");

            let choice = read_input("Choose an option:")?;

            match choice.trim() {
                "1" => list_tasks(&tasks),
                "2" => {
                    let task = read_input("Enter a new task: ")?;
                    tasks.push(task);
                    save_tasks(&tasks)?;
                    println!("Task added.");
                }
                "3" => {
                    list_tasks(&tasks);
                    let index = read_input("Enter task number to remove: ")?;
                    if let Ok(num) = index.trim().parse::<usize>() {
                        if num > 0 && num <= tasks.len() {
                            tasks.remove(num - 1);
                            save_tasks(&tasks)?;
                            println!("Task removed.");
                        } else {
                            println!("Invalid task number!.");
                        }
                    } else {
                        println!("Please enter a valid number.");
                    }
                }
                "4" => {
                    println!("Goodbye!");
                    break;
                }
                _ => println!("Invalid choice"),
            }
        }
    }
    Ok(())
}

fn read_input(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn list_tasks(tasks: &[String]) {
    if tasks.is_empty() {
        println!("No tasks yet!");
    } else {
        for (i, task) in tasks.iter().enumerate() {
            println!("{}. {}", i + 1, task);
        }
    }
}

fn load_tasks() -> Result<Vec<String>, io::Error> {
    if !Path::new(FILE_NAME).exists() {
        File::create(FILE_NAME)?;
    }

    let file = File::open(FILE_NAME)?;
    let reader = BufReader::new(file);

    let tasks = reader.lines().filter_map(Result::ok).collect();

    Ok(tasks)
}

fn save_tasks(tasks: &[String]) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(FILE_NAME)?;

    for task in tasks {
        writeln!(file, "{}", task)?;
    }

    Ok(())
}
