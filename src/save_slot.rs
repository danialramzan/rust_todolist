use serde::{Deserialize, Serialize};
use crate::todo_list::ToDoList;

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveSlot {
    id: u32,
    to_do_list: ToDoList,
    timestamp: String,
}

impl SaveSlot {


    // constructor
    pub fn new(id: u32, to_do_list: ToDoList, timestamp: String) -> Self {
        SaveSlot {
            id,
            to_do_list,
            timestamp,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn get_to_do_list(&self) -> &ToDoList {
        &self.to_do_list
    }

    pub fn set_to_do_list(&mut self, to_do_list: ToDoList) {
        self.to_do_list = to_do_list;
    }

    pub fn get_timestamp(&self) -> &str {
        &self.timestamp
    }

    pub fn set_timestamp(&mut self, timestamp: String) {
        self.timestamp = timestamp;
    }
}