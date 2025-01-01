# Simplo
A simple CLI todo list, made entirely using Rust. You can add tasks, remove them, or clear the todo list whenever needed. As opposed to other similar todo lists, the todo list saves the tasks without the need to have it constantly running, allowing users to do whatever they want in the meantime before coming back and knocking off a task. Minimalist and expressive for all users. 

## How it works
### Setting Up
To run the code:

Compile using `rustc` and 'cargo build' and then call the executable along with whatever argument is needed. Alternatively, `cargo run` should work but not as well. 

### Adding tasks
You can add tasks by calling `./main [tasks]`. You can have as many *space-separated* tasks as needed, and each will be added separately. For example, `./main 
