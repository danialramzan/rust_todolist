mod todo_list;
mod todo_list_wrapper;
pub mod save_slot;

use std::io;
use std::io::Write;
use std::process;
use crate::todo_list_wrapper::ToDoListWrapper;


fn main() {

    // structure:

    // splash has options
    // add (implemented using hashmap with lowest available number)
    // list
    // delete
    // edit task

    use todo_list::ToDoList;
    use todo_list_wrapper::ToDoListWrapper;
    let mut to_do_list = ToDoList::new();
    let mut wrapper = ToDoListWrapper::new(&mut to_do_list);

    // panic!("at the disco");

    println!("\nWelcome to my to-do app!");
    prompt(&mut wrapper);
}

pub fn prompt(wrapper: &mut ToDoListWrapper) {
    let mut number = -1;

    while (![1, 2, 3, 4, 5, 6, 7].contains(&number)) {
        println!("\n  (1) List current tasks");
        println!("  (2) Add task");
        println!("  (3) Remove task");
        println!("  (4) Edit task");
        println!("  (5) Load");
        println!("  (6) Save");
        println!("  (7) Exit program");
        let mut input = String::new();
        print!("\nPlease input a number from 1-7 inclusive to make a selection: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut input).expect("Unrecoverable Error Encountered!");

        number = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
    }

    // at this point we know that number is a valid input

    match number {
        // case statements
        1 => wrapper.list(),
        2 => wrapper.add(),
        3 => wrapper.remove(),
        4 => wrapper.edit(),
        5 => wrapper.load(),
        6 => wrapper.save(),
        7 => process::exit(0),
        _ => {}
    }

}
