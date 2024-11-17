use super::todo_list::ToDoList;
use std::io;

pub struct ToDoListWrapper<'a> {
    todo_list: &'a mut ToDoList,
}

impl<'a> ToDoListWrapper<'a> {
    pub fn new(todo_list: &'a mut ToDoList) -> Self {
        ToDoListWrapper { todo_list }
    }


    // list function

    pub fn list(&mut self) {
        // let mut input = String::new();
        // print!("Enter a description for the new task: ");
        // io::stdout().flush().expect("Failed to flush output"); // Ensures the prompt is displayed before user input
        //
        // io::stdin()
        //     .read_line(&mut input)
        //     .expect("Failed to read input");
        //
        // let description = input.trim().to_string();
        // if !description.is_empty() {
        //     self.todo_list.add_task(description);
        //     println!("Task added successfully!");
        // } else {
        //     println!("Task description cannot be empty.");
        // }
    }
    pub fn add(&mut self) {
        // let mut input = String::new();
        // print!("Enter a description for the new task: ");
        // io::stdout().flush().expect("Failed to flush output"); // Ensures the prompt is displayed before user input
        //
        // io::stdin()
        //     .read_line(&mut input)
        //     .expect("Failed to read input");
        //
        // let description = input.trim().to_string();
        // if !description.is_empty() {
        //     self.todo_list.add_task(description);
        //     println!("Task added successfully!");
        // } else {
        //     println!("Task description cannot be empty.");
        // }
    }

    pub fn remove(&mut self) {
        // let mut input = String::new();
        // print!("Enter a description for the new task: ");
        // io::stdout().flush().expect("Failed to flush output"); // Ensures the prompt is displayed before user input
        //
        // io::stdin()
        //     .read_line(&mut input)
        //     .expect("Failed to read input");
        //
        // let description = input.trim().to_string();
        // if !description.is_empty() {
        //     self.todo_list.add_task(description);
        //     println!("Task added successfully!");
        // } else {
        //     println!("Task description cannot be empty.");
        // }
    }

    pub fn edit(&mut self) {
        // let mut input = String::new();
        // print!("Enter a description for the new task: ");
        // io::stdout().flush().expect("Failed to flush output"); // Ensures the prompt is displayed before user input
        //
        // io::stdin()
        //     .read_line(&mut input)
        //     .expect("Failed to read input");
        //
        // let description = input.trim().to_string();
        // if !description.is_empty() {
        //     self.todo_list.add_task(description);
        //     println!("Task added successfully!");
        // } else {
        //     println!("Task description cannot be empty.");
        // }
    }


    // Methods for TodoListWrapper can go here
}
