use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

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

}



