use std::collections::HashMap;

pub struct ToDoList {
    tasks: HashMap<u32, String>,
    task_number: u32
}

impl ToDoList {

    // constructor
    pub fn new() -> Self {
        ToDoList {
            tasks: HashMap::new(),
            task_number: 1,
        }
    }






}