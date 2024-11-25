use std::collections::BTreeMap;

pub struct ToDoList {
    tasks: BTreeMap<u32, String>,
    task_number: u32
}

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

    pub fn save(&mut self) -> u32 {

        for (task_index, task) in self.get_tasks().iter() {
            // add task_index, task to save file
            println!("{}: {}", task_index, task);
            }
        return 3;
        }

        // save all entries and their indexes into json file
        // save and return slot number

    }



