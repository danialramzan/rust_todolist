### Rust To-Do List App

#### A job I applied to had Rust as a nice-to-have so I decided now was as good a time as any to learn the language. I did this by making a to-do list using best coding practices I learned from my Software Construction class (other than tests, for now). 

## Features

- **Add Task:** Allows the user to add tasks to the todo list.
- **List Tasks:** Displays all the tasks in the list, showing their indices and descriptions.
- **Remove Task:** Enables the user to remove a task by specifying its index.
- **Edit Task:** Allows access to the underlying data structure to edit a task.

## Getting Started

### Prerequisites

- Rust installed on your machine (use [rustup](https://rustup.rs/) for installation).

### Building the Project

1. Clone the repository:
    ```bash
    git clone https://github.com/danialramzan/rust_todolist.git
    cd rust_todolist
    ```

2. Build the project:
    ```bash
    cargo build
    ```

3. Run the project:
    ```bash
    cargo run
    ```
## Planned Future Implementations

  - Testing: Implement unit and integration tests to ensure code correctness and reliability.
  - GUI: Develop a graphical user interface to provide a more user-friendly way to interact with the Todo List.




## Code Structure

### `ToDoList` (Primary Struct)

The `ToDoList` struct manages the underlying data structure used to store tasks. It uses a `BTreeMap` to keep tasks in a sorted order. Key fields and methods:

- **Fields**:
  - `tasks`: `BTreeMap<u32, String>` - A map storing task indices and their corresponding descriptions.
  - `task_number`: `u32` - A counter to store the current task index.
  
- **Methods**:
  - `new()`: Initializes a new `ToDoList`.
  - `get_tasks()`: Returns a reference to the task map.
  - `get_task_number()`: Returns the current task number.
  - `set_tasks()`: Updates the task map.
  - `set_task_number()`: Updates the task counter.
  - `add_edit_task()`: Adds or edits a task.
  - `remove_task()`: Removes a task by its index.

### `ToDoListWrapper` (Interaction Layer)

The `ToDoListWrapper` struct provides an interface for interacting with a `ToDoList`. It handles input/output and user prompts.

- **Methods**:
  - `list()`: Lists all tasks, using `get_tasks()` to retrieve task data.
  - `prompt_main_menu()`: Prompts the user to return to the main menu or exit.
  - `add()`: Adds a new task with user input.
  - `remove()`: Removes a task based on a user-provided index.
  - `edit()`: Edits an existing task based on a user-provided index.
 
