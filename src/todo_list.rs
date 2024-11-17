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
    //
    // pub fn add_task(&mut self, description: String) {
    //     self.tasks.insert(self.task_number, description);
    //     println!("Task {} added.", self.task_number);
    //     self.task_number += 1;
    // }






}