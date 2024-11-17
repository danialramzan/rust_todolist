mod to_do_list;

fn main() {

    // structure:

    // splash has options
    // add (implented using hashmap with lowest available number)
    // list
    // delete
    // edit task

    use to_do_list::ToDoList;
    let mut to_do_list = ToDoList::new();


    println!("Welcome to my to-do app");
    println!("Please enter a number to make a section:");
    println!("  (1) List current tasks");
    println!("  (2) Add task");
    println!("  (3) Remove task");
    println!("  (4) Edit task");
}
