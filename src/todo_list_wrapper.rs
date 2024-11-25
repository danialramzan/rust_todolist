use super::todo_list::ToDoList;
use std::{io, process};
use std::io::Write;
use std::fs::File;
use serde_json;

pub struct ToDoListWrapper<'a> {
    todo_list: &'a mut ToDoList,
}

impl<'a> ToDoListWrapper<'a> {
    pub fn new(todo_list: &'a mut ToDoList) -> Self {
        ToDoListWrapper { todo_list }
    }


    // list function

    pub fn list(&mut self) {
        // uses getter
        let map = self.todo_list.get_tasks();
        if (map.len() == 0) {
            println!("No tasks found");
        } else {
            for (task_index, task) in map.iter() {
                println!("{}: {}", task_index, task);
            }
        }
        self.prompt_main_menu();
    }

    fn prompt_main_menu(&mut self) {
        let mut input = String::new();
        while !["y", "n", "Y", "N"].contains(&input.trim()) {
            print!("Would you like to go back to the main menu? (y/n): ");
            io::stdout().flush().expect("Failed to flush stdout");
            input.clear();
            io::stdin().read_line(&mut input).expect("Unrecoverable Error Encountered!");
        }
        if input.trim().eq_ignore_ascii_case("y") {
            crate::prompt(self);
        } else {
            process::exit(0);
        }
    }

    pub fn add(&mut self) {

        let mut input = String::new();

        while input.trim().to_string().is_empty() {
            print!("Enter a non-empty description for task with index {}: ", self.todo_list.get_task_number());
            io::stdout().flush().expect("Failed to flush stdout");
            input.clear();
            io::stdin().read_line(&mut input).expect("Unrecoverable Error Encountered!");
        }
        let description = input.trim().to_string();
        self.todo_list.add_edit_task(self.todo_list.get_task_number(), description);
        self.todo_list.set_task_number(self.todo_list.get_task_number() + 1);

        println!("Task added successfully!");

        self.prompt_main_menu();
    }

    pub fn remove(&mut self) {

        // technically this should not fail for the first 2,147,483,647 items methinks
        let mut input = String::new();
        let mut number:i32 = 0;
        while !self.todo_list.get_tasks().contains_key(&(number as u32)) {

            print!("Please enter a valid index to remove or enter -1 to go back to main menu: ");
            io::stdout().flush().expect("Failed to flush stdout");
            input.clear();
            io::stdin().read_line(&mut input).expect("Unrecoverable Error Encountered!");

            number = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue
            };

            if (number == -1) {
                crate::prompt(self);
            }
        }

        self.todo_list.remove_task(number as u32);
        println!("Task removed successfully!");
        self.prompt_main_menu();
    }

    pub fn edit(&mut self) {

        // remove code

        // technically this should not fail for the first 2,147,483,647 items methinks
        let mut input = String::new();
        let mut number:i32 = 0;
        while !self.todo_list.get_tasks().contains_key(&(number as u32)) {

            print!("Please enter a valid index to edit or enter -1 to go back to main menu: ");
            io::stdout().flush().expect("Failed to flush stdout");
            input.clear();
            io::stdin().read_line(&mut input).expect("Unrecoverable Error Encountered!");

            number = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue
            };

            if (number == -1) {
                crate::prompt(self);
            }
        }

        let mut input2 = String::new();

        while input2.trim().to_string().is_empty() {
            print!("Enter a non-empty description for task with index {}: ", number);
            io::stdout().flush().expect("Failed to flush stdout");
            input2.clear();
            io::stdin().read_line(&mut input2).expect("Unrecoverable Error Encountered!");
        }
        let description = input2.trim().to_string();
        self.todo_list.add_edit_task(number as u32, description);

        println!("Task edited successfully!");

        self.prompt_main_menu();

    }

    pub fn load(&mut self) {

        // check if json is empty
        // if empty say no load options available
        // else show list and let them select
        // call load in todo_list (replaces current btreelist with contents of json)


    }

    pub fn save(&mut self) {

        if (self.todo_list.get_tasks().iter().len() == 0) {
            println!("No tasks found to save!");
        } else{
            let slot_num = self.todo_list.save();
            println!("Saved successfully in slot{}!", slot_num);
        }

        self.prompt_main_menu();
    }


}
