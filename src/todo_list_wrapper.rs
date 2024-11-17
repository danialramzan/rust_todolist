use super::todo_list::ToDoList;

pub struct ToDoListWrapper<'a> {
    todo_list: &'a mut ToDoList,
}

impl<'a> ToDoListWrapper<'a> {
    // Methods for TodoListWrapper
}