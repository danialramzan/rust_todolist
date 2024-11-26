use crate::save_slot::SaveSlot;
use super::todo_list::{ToDoList};
use std::{io, process};
use std::io::Write;
use std::io::Read;
use chrono::Local;
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
        self.view_tasks();
        self.prompt_main_menu();
    }

    pub fn view_tasks(&mut self) {
        let map = self.todo_list.get_tasks();
        if (map.len() == 0) {
            println!("No tasks found");
        } else {
            for (task_index, task) in map.iter() {
                println!("{}: {}", task_index, task);
            }
        }
    }

    pub fn prompt_main_menu(&mut self) {
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
            if !self.todo_list.is_empty() {
                println!("\nCurrent contents of To-Do List: ");
            }
            self.view_tasks();
            if self.todo_list.is_empty() {
                self.prompt_main_menu()
            }

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
        self.todo_list.reindex();
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

        if std::fs::metadata("data/save_file.json").map(|m| m.len()).unwrap_or(0) == 0 {
            println!("No saves found to load from!");
        } else {
            let save_file = Self::load_from_file();

            let indexes: Vec<u32> = save_file.iter().map(|slot| slot.get_id()).collect();
            let mut input = String::new();
            let mut number:i32 = 0; // this deviates from convention a bit...


            while (!indexes.contains(&(number as u32))) {
                println!("\nAvailable save slots:");
                for slot in &save_file {
                    println!("Slot ID: {}, Timestamp: {}", slot.get_id(), slot.get_timestamp());
                }
                print!("\nEnter the index for the save slot you would like to load. Enter -1 to go back: ");

                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin().read_line(&mut input).expect("Unrecoverable Error Encountered!");
                number = match input.trim().parse() {
                    Ok(num) => {
                        if num == -1 {
                            crate::prompt(self); // Call `prompt` and exit the current function
                        }
                        num
                    }
                    Err(_) => continue,
                };
            }

            let slot = save_file.iter().find(|slot| slot.get_id() == number as u32).unwrap();
            self.todo_list.set_tasks(slot.get_to_do_list().get_tasks().clone());
            self.todo_list.set_task_number(slot.get_to_do_list().get_task_number());


            println!("Successfully loaded data from slot {}", number);
        }

        self.prompt_main_menu();
    }

    fn load_from_file() -> Vec<SaveSlot> {
        let file_path = "data/save_file.json";
        let mut save_file: Vec<SaveSlot> = Vec::new();

        // load the json data
        let mut content = String::new();
        if let Ok(mut file) = std::fs::File::open(file_path) {
            if let Ok(_) = file.read_to_string(&mut content) {
                if let Ok(data) = serde_json::from_str(&content) {
                    save_file = data;
                }
            }
        }
        save_file
    }

    pub fn save(&mut self) {

        if (self.todo_list.get_tasks().iter().len() == 0) {
            println!("No tasks found to save!");
        } else {

            let mut save_file = Self::load_from_file();

            // get curr max slot id by iter trough json data
            let max_slot_id = save_file.iter().map(|slot| slot.get_id()).max().unwrap_or_else(|| 0);

            // make max slot
            let new_slot = SaveSlot::new(max_slot_id + 1, self.todo_list.clone(), Local::now().to_rfc3339());

            // add the new slot to the loaded json data
            save_file.push(new_slot);

            // save save save
            let json_data = match serde_json::to_string_pretty(&save_file) {
                Ok(json) => json,
                Err(_) => {return;}
            };

            // boilerplate
            if let Err(_) = std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open("data/save_file.json")
                .and_then(|mut file| file.write_all(json_data.as_bytes()))
            {}


            let slot_num = max_slot_id + 1;
            println!("Saved successfully in slot {}!", slot_num);
        }

        self.prompt_main_menu();
    }


}
