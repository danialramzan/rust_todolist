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

    pub fn get_tasks(&self) -> &HashMap<u32, String> {
        &self.tasks
    }

    pub fn set_tasks(&mut self, new_tasks: HashMap<u32, String>) {
        self.tasks = new_tasks;
    }

    pub fn add_edit_task(&mut self, key: u32, value: String) {
        self.tasks.insert(key, value);
    }

    pub fn remove_task(&mut self, key: u32) {
        self.tasks.remove(&key);
    }


}