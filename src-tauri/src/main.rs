// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
struct Todo {
    title: String,
    status: String,
    content: String,
    color: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

struct DB;

impl DB {
    async fn initialize_db() -> surrealdb::Result<Surreal<Client>> {
        // Connect to the server
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

        // Signin as a namespace, database, or root user
        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await?;

        // Select a specific namespace / database
        db.use_ns("test").use_db("test").await?;

        Ok(db)
    }
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_todo, get_todos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn create_todo(todo: Todo) -> String {
    let db = DB::initialize_db()
        .await
        .expect("Error initializing the DB");

    let created: Record = db
        .create("todo")
        .content(todo)
        .await
        .expect("To create a record");

    format!("{}", created.id)
}

#[tauri::command]
async fn get_todos() -> Vec<Todo> {
    let db = DB::initialize_db()
        .await
        .expect("Error initializing the DB");

    db.select("todo")
        .await
        .expect("The Todo table couldn't be fetched")

}
