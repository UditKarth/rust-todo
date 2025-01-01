# Simplo
A simple CLI todo list, made entirely using Rust. You can add tasks, remove them, or clear the todo list whenever needed. As opposed to other similar todo lists, the todo list saves the tasks without the need to have it constantly running, allowing users to do whatever they want in the meantime before coming back and knocking off a task. Minimalist and expressive for all users. 

## How it works
### Setting Up
To run the code:

Compile using `rustc` and 'cargo build' and then call the executable along with whatever argument is needed. Alternatively, `cargo run` should work but not as well. 

### Adding tasks
You can add tasks by calling `./main [tasks]`. You can have as many *space-separated* tasks as needed, and each will be added separately. For example, `./main task1 task2` will be added separately. If you want longer tasks, you can surround the task in quotation marks; for example `./main add "Take out trash"` would be added as "Take out trash", and not separate entries. 

### Removing tasks
You can remove tasks when you finish them by using `./main remove [task name]`. You can also use ./main remove [task number]`. If you want to remove multiple at once, you can do so using any combination of task name or task number. Task name removal is currently case insensitive but spelling sensitive.

### Listing tasks
You can list all tasks by using `./main list`

### Clearing list
You can also clear all tasks by using `main ./clear`. 
