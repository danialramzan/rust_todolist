mod to_do_list;
use std::io;

fn main() {

    // structure:

    // splash has options
    // add (implented using hashmap with lowest available number)
    // list
    // delete
    // edit task

    use to_do_list::ToDoList;
    let mut to_do_list = ToDoList::new();
    // panic!("at the disco");


    println!("Welcome to my to-do app");
    println!("Please enter a number to make a section:");
    println!("  (1) List current tasks");
    println!("  (2) Add task");
    println!("  (3) Remove task");
    println!("  (4) Edit task");

    let mut number = -1;

    while (![1, 2, 3, 4].contains(&number)) {
        let mut input = String::new();
        println!("Please input a number from 1-4 inclusive: ");
        io::stdin().read_line(&mut input).expect("Unrecoverable Error Encountered!");

        number = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
    }

    // at this point we know that number is a valid input


    // if n % 15 == 0 {
    //         println!("fizzbuzz");
    //     } else if n % 3 == 0 {
    //         println!("fizz");
    //     } else if n % 5 == 0 {
    //         println!("buzz");
    //     } else {
    //         println!("{}", n);
    //     }
    // }


    // match number {
    //     // Match a single value
    //     1 => println!("One!"),
    //     // Match several values
    //     2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
    //     // TODO ^ Try adding 13 to the list of prime values
    //     // Match an inclusive range
    //     13..=19 => println!("A teen"),
    //     // Handle the rest of cases
    //     _ => println!("Ain't special"),
    //     // TODO ^ Try commenting out this catch-all arm
    // }


}
