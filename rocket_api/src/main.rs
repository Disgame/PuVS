#[macro_use]
extern crate rocket;

use serde::{Serialize, Deserialize};

use rocket::{response::status, serde::json::Json};


#[derive(Serialize, Deserialize)]
pub struct Todo {
    name: String,
    completed: bool,
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, Todo List!"
}

#[get("/todos")] // <- returns a list of todos
fn get_todos() -> &'static str {
    "Here are all the todos"
}

#[get("/todos/<name>")] // <- returns a single todo
fn get_todo_by_name(name: &str) -> String {
    format!("Here is the todo with name: {}", name)
}

#[post("/todos", data = "<todo>")] // <- creates a new todo
fn create_todo(todo: Json<Todo>) -> status::Created<Json<Todo>> {
    status::Created::new("/todos").body(todo)
}

#[put("/todos", data = "<todo>")] // <- updates a todo
fn update_todo(todo: Json<Todo>) -> Json<Todo> {    
    todo
}

#[delete("/todos/<name>")] // <- deletes a todo
fn delete_todo_by_name(name: &str) -> status::NoContent {
    status::NoContent
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/", routes![get_todos])
        .mount("/", routes![get_todo_by_name])
        .mount("/", routes![create_todo])
        .mount("/", routes![update_todo])
        .mount("/", routes![delete_todo_by_name])
}