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

    // Setter for tasks (replaces the entire tasks map)
    pub fn set_tasks(&mut self, new_tasks: HashMap<u32, String>) {
        self.tasks = new_tasks;
    }
    //
    // pub fn add_task(&mut self, description: String) {
    //     self.tasks.insert(self.task_number, description);
    //     println!("Task {} added.", self.task_number);
    //     self.task_number += 1;
    // }






}