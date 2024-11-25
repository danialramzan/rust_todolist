use crate::ToDoListWrapper;
use std::collections::BTreeMap;
use std::io;
use chrono::Local; // For timestamp generation
use serde::{Deserialize, Serialize}; // For serialization
use std::io::{Read, Write};
use std::iter::Map;
use std::slice::Iter;
use crate::save_slot::SaveSlot;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToDoList {
    tasks: BTreeMap<u32, String>,
    task_number: u32
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct SaveSlot {
//     id: u32,
//     to_do_list: ToDoList,
//     timestamp: String,
// }

impl ToDoList {

    // constructor
    pub fn new() -> Self {
        ToDoList {
            tasks: BTreeMap::new(),
            task_number: 1,
        }
    }

    pub fn get_tasks(&self) -> &BTreeMap<u32, String> {
        &self.tasks
    }

    pub fn get_task_number(&self) -> u32 {
        self.task_number
    }


    pub fn set_tasks(&mut self, new_tasks: BTreeMap<u32, String>) {
        self.tasks = new_tasks;
    }

    pub fn set_task_number(&mut self, value: u32) {
        self.task_number = value;
    }

    pub fn add_edit_task(&mut self, key: u32, value: String) {
        self.tasks.insert(key, value);
    }

    pub fn remove_task(&mut self, key: u32) {
        self.tasks.remove(&key);
    }

    pub fn load(&mut self) -> i32 {

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

        let indexes: Vec<u32> = save_file.iter().map(|slot| slot.get_id()).collect();
        let mut input = String::new();
        let mut number:i32 = 0; // this deviates from convention a bit...


        while (!indexes.contains(&(number as u32))) {
            println!("Enter the index for the save slot you would like to load. Enter -1 to go back.");
            println!("Available save slots:\n");
            for slot in &save_file {
                println!("Slot ID: {}, Timestamp: {}", slot.get_id(), slot.get_timestamp());
            }

            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin().read_line(&mut input).expect("Unrecoverable Error Encountered!");

            number = match input.trim().parse() {
                Ok(num) => {
                    if num == -1 {
                        crate::prompt(&mut ToDoListWrapper::new(self)); // Call `prompt` and exit the current function
                    }
                    num
                }
                Err(_) => continue,
            };


        }

        // number is valid

        return number;


        }

    pub fn save(&mut self) -> i32 {

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

        // get curr max slot id by iter trough json data
        let max_slot_id = save_file.iter().map(|slot| slot.get_id()).max().unwrap_or_else(|| 1);

        // make max slot
        let new_slot = SaveSlot::new(max_slot_id + 1, self.clone(), Local::now().to_rfc3339());

        // add the new slot to the loaded json data
        save_file.push(new_slot);

        // save save save
        let json_data = match serde_json::to_string_pretty(&save_file) {
            Ok(json) => json,
            Err(_) => {
                return -1;
            }
        };

        // boilerplate
        if let Err(_) = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)
            .and_then(|mut file| file.write_all(json_data.as_bytes()))
        {
            return -1;
        }


        let json_data = match serde_json::to_string_pretty(&self.tasks) {
            Ok(json) => json,
            Err(_) => {
                return -1;
            }
        };

        return (max_slot_id + 1) as i32;
    }
    }



