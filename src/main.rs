use actix_web::{get, web, App, HttpServer};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;


mod todolist;
use crate::todolist::backend_services::configure_services;

struct AppState {
    to_do_list_entries: Mutex<Vec<ToDoListEntry>>
}

#[derive(Serialize, Deserialize, Clone)]
struct ToDoListEntry {
    id: i32,
    date: i64,
    title: String,
}

#[get("/")]
async fn index() -> String {
    "this is a health check".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(
        AppState {
            to_do_list_entries: Mutex::new(vec![])
        }
    );

    HttpServer::new(move|| {
       App::new()
           .app_data(app_data.clone())
           .service(index)
           .configure(configure_services)
    })
        .bind(("0.0.0.0", 8090))?
        .run()
        .await
}
