extern crate mongodb;

use mongodb::bson::{doc, Document};
use mongodb::{Client, options::ClientOptions};
use rocket::futures::StreamExt;

async fn connect() -> mongodb::Database {
    let client_options = ClientOptions::parse("mongodb://todo-db:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("todo");
    db.create_collection("todos", None).await.unwrap();
    db
}

pub async fn get_collection() -> mongodb::Collection<Document> {
    let db = connect().await;
    db.collection::<Document>("todos")
}

pub async fn fetch_todos() -> Vec<String> {
    let collection = get_collection().await;
    let cursor_result = collection.find(None, None).await;
    let mut todos = Vec::new();
   
    if let Ok(mut cursor) = cursor_result {
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let todo = document.get_str("todo").unwrap();
                    todos.push(todo.to_string());
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
    
    todos
}

pub async fn create_todo(todo: String) {
    let collection = get_collection().await;
    let document = doc! {
        "todo": todo
    };
    collection.insert_one(document, None).await.unwrap();
}

pub async fn delete_todo_by_name(todo: String) {
    let collection = get_collection();
    collection.await.delete_one(doc! { "todo": todo }, None).await.unwrap();
}