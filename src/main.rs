use std::env;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader, BufWriter};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Usage: ./main add <task1>,<task2>,...");
                return;
            }
            let tasks_string = &args[2];
            let tasks: Vec<&str> = tasks_string.split(',').map(|s| s.trim()).collect();
            let mut num = get_task_count();
            for task in tasks {
                add(task, &mut num);
            }
        }
        "remove" => {
            if args.len() < 3 {
                println!("Usage: ./main remove <task1> <task2> ...");
                return;
            }
            let remove_options = &args[2..]; 
            remove(remove_options); 
        }
        "list" => list(),
        "clear" => clear_list(),
        _ => print_usage(),
    }
}

fn get_task_count() -> i32 {
    let file = match File::open("todo.txt") {
        Ok(file) => file,
        Err(_) => return 1,
    };

    let reader = BufReader::new(file);
    let mut max_num = 0;
    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            if let Some(colon_index) = line.find(':') {
                if let Ok(num) = line[..colon_index].parse::<i32>() {
                    max_num = max_num.max(num);
                }
            }
        }
    }
    max_num + 1
}

fn list() {
    let file = match File::open("todo.txt") {
        Ok(file) => file,
        Err(_) => {
            println!("No tasks found.");
            return;
        }
    };
    let reader = BufReader::new(file);

    for line_result in reader.lines().enumerate() {
        match line_result {
            (_i, Ok(line)) => println!("{}",  line),
            (_, Err(e)) => eprintln!("Error reading line: {}", e),
        }
    }
}

fn add(new_task: &str, num: &mut i32) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("todo.txt")
        .expect("Unable to open file");

    let mut writer = BufWriter::new(&mut file);
    writeln!(writer, "{}: {}", *num, new_task).expect("Unable to write to file");
    *num += 1;
}

fn remove(remove_options: &[String]) { 
    let mut lines: Vec<String> = Vec::new();
    let file = match File::open("todo.txt") {
        Ok(file) => file,
        Err(_) => {
            println!("No tasks found to remove.");
            return;
        }
    };
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => lines.push(line),
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    let mut removed = false;
    let mut new_lines: Vec<String> = Vec::new();
    let mut new_num = 1;

    for line in lines {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() == 2 {
            let task_num_str = parts[0].trim();
            let task_text = parts[1].trim();

            let should_remove = remove_options.iter().any(|option| {
                task_text.contains(option) || task_num_str == option
            });

            if should_remove {
                removed = true;
            } else {
                new_lines.push(format!("{}: {}", new_num, task_text));
                new_num += 1;
            }
        } else {
            eprintln!("Invalid line format: {}", line);
        }
    }

    if removed {
        let mut file = File::create("todo.txt").expect("Unable to open file");
        let mut writer = BufWriter::new(&mut file);
        for line in new_lines {
            writeln!(writer, "{}", line).expect("Unable to write to file");
        }
        println!("Task(s) removed.");
    } else {
        println!("Task not found.");
    }
}

fn clear_list() {
    File::create("todo.txt").expect("Unable to create file");
    println!("To-Do list cleared.");
}

fn print_usage() {
    println!("Usage: ./main <command> [arguments]");
    println!("Commands:");
    println!("  add <task1> <task2> ...: Add tasks to the list");
    println!("  remove <task_number_or_name>: Remove a task from the list");
    println!("  list: List all tasks");
    println!("  clear: Clear the task list");
}
