#[macro_use]
extern crate rocket;

use std::future::IntoFuture;

use rocket::{
    fairing::{
        Fairing, 
        Info, 
        Kind
    },
    http::{
        Header,
        Status
    },
    Request, 
    Response,
    response::{
        status,
        Redirect
    }
};
use serde_json::json;

mod db;
/*
https://stackoverflow.com/questions/62412361/how-to-set-up-cors-or-options-for-rocket-rs/69342225#69342225 
*/
pub struct CORS;
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET, POST, PUT, PATCH, DELETE, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

/* API Routes*/
#[get("/")]
fn hello() -> &'static str {
    "Hello, this is my Todos Application, see them at /todos!\nCreate a new one with POST /todos/<name>\nDelete one with DELETE /todos/<name>"
}

#[get("/todos")] // <- returns a list of todos
async fn get_todos() -> status::Custom<String> {
    let todos = db::fetch_todos().await;
    status::Custom(Status::Ok, todos.join(", "))
}

#[post("/todos/<name>")] // <- creates a new todo
async fn create_todo(name: &str) -> status::Custom<String> {
    db::create_todo(name.to_string()).await;
    status::Custom(Status::Ok, name.to_string())
}

#[delete("/todos/<name>")] // <- deletes a todo
async fn delete_todo_by_name(name: &str) -> status::Custom<String> {
    db::delete_todo_by_name(name.to_string()).await;
    status::Custom(Status::Ok, name.to_string())
}

/* Catchers */
#[catch(default)]
fn erro_redirect() -> rocket::response::Redirect {
    Redirect::to("/")
}

/* Rocket Application */
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![hello, get_todos, create_todo, delete_todo_by_name])
        .register("/", catchers![erro_redirect])
}