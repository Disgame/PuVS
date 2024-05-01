use reqwasm::{http::Request, Error};


const BASE_URL: &str = "http://localhost:8000";

pub async fn fetch_todos() -> Vec<String> {
    let response = Request::get(&format!("{BASE_URL}/todos"))
        .send()
        .await
        .unwrap();
    let todos: Vec<String> = response.json().await.unwrap();
    todos
}

pub async fn create_todo(name: &str) -> Result<String, Error> {
    let response = Request::post(&format!("{}/todos/{}", BASE_URL, name))
        .send()
        .await?;
    let message = response.text().await?;
    Ok(message)
}

pub async fn delete_todo(name: &str) -> Result<String, Error> {
    let response = Request::delete(&format!("{}/todos/{}", BASE_URL, name))
        .send()
        .await?;
    let message = response.text().await?;
    Ok(message)
}