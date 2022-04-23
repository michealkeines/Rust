extern crate todolist_parser;

use todolist_parser::TodoList;

fn main() {
    let todo = TodoList::get_todos("examples/todo");
    match todo {
        Ok(list) => println!("{:?}", list),
        Err(e)=> {
            println!("{}", e.description());
            println!("{:?}", e)
        }
    }
}