## Actix-Web REST API (v1)

#### Directory Structure

```bash
├── Cargo.lock
├── Cargo.toml
├── README.md
├── src
│   ├── main.rs
│   └── todolist
│       ├── backend_services.rs
│       ├── data_models.rs
│       └── mod.rs
```

`Cargo.toml`

```toml
[package]
name = "actix-rest-api"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
actix-web = "4.3.1"
serde = { version = "1.0.163", features = ["derive"] }
serde_json = "1.0.96"
```

## Files

`src/main.rs`

```rust
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
```

`src/todolist/mod.rs`

```rust
pub mod backend_services;
mod data_models;
```

`src/todolist/data_models.rs`

```rust
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateEntryData {
    pub title: String,
    pub date: i64,
}

#[derive(Deserialize, Clone)]
pub struct UpdateEntryData {
    pub title: String,
    pub date: i64,
}
```

`src/todolist/backend_services.rs`

```rust

use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
use actix_web::web::ServiceConfig;
use crate::{AppState, ToDoListEntry};
use super::data_models::{CreateEntryData, UpdateEntryData};

#[get("/todolist/entries")]
async fn get_entries(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.to_do_list_entries.lock().unwrap().to_vec())
}

#[post("/todolist/entries")]
async fn create_entry(data: web::Data<AppState>, param_obj: web::Json<CreateEntryData>) -> impl Responder {
    let mut to_do_list_entries = data.to_do_list_entries.lock().unwrap();
    let mut max_id: i32 = 0;
    for i in 0..to_do_list_entries.len() {
        if to_do_list_entries[i].id > max_id {
            max_id = to_do_list_entries[i].id
        }
    }
    to_do_list_entries.push(ToDoListEntry{
        id: max_id + 1,
        title: param_obj.title.clone(),
        date: param_obj.date,
    });
    HttpResponse::Ok().json(to_do_list_entries.to_vec())
}

#[put("/todolist/entries/{id}")]
async fn update_entry(data: web::Data<AppState>, path: web::Path<i32>, param_obj: web::Json<UpdateEntryData>) -> impl Responder {
    let id = path.into_inner();
    let mut to_do_list_entries = data.to_do_list_entries.lock().unwrap();

    for i in 0..to_do_list_entries.len() {
        if to_do_list_entries[i].id == id {
            to_do_list_entries[i].title = param_obj.title.clone();
            break;
        }
    }
    HttpResponse::Ok().json(to_do_list_entries.to_vec())
}


#[delete("/todolist/entries/{id}")]
async fn delete_entry(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let mut to_do_list_entries = data.to_do_list_entries.lock().unwrap();

    *to_do_list_entries = to_do_list_entries.to_vec().into_iter().filter(|x| x.id != id).collect();

    HttpResponse::Ok().json(to_do_list_entries.to_vec())
}


pub fn configure_services(cfg: &mut web::ServiceConfig) {
    cfg.service(get_entries).service(create_entry).service(update_entry).service(delete_entry);
}
```

## Testing REST API Endpoints

#### Fetch Records

```bash
curl -X GET http://localhost:8090/todolist/entries 2>/dev/null | python3.8 -m json.tool
[]
```

#### Create Record

```bash
# curl -H "content-type: application/json" -H "accept: application/json" -X POST http://localhost:8090/todolist/entries -d '{"title": "date with wife", "date":1685308792}' 2> /dev/null | python3.8 -m json.tool
[
    {
        "id": 1,
        "date": 1685308792,
        "title": "date with wife"
    }
]
```

#### Create Record

```bash
# curl -H "content-type: application/json" -H "accept: application/json" -X POST http://localhost:8090/todolist/entries -d '{"title": "pick up kids", "date":1685308916}' 2> /dev/null | python3.8 -m json.tool
[
    {
        "id": 1,
        "date": 1685308792,
        "title": "date with wife"
    },
    {
        "id": 2,
        "date": 1685308916,
        "title": "pick up kids"
    }
]
```

#### Create Record

```bash
# curl -H "content-type: application/json" -H "accept: application/json" -X POST http://localhost:8090/todolist/entries -d '{"title": "pick up groceries", "date":1685309090}' 2> /dev/null | python3.8 -m json.tool
[
    {
        "id": 1,
        "date": 1685308792,
        "title": "date with wife"
    },
    {
        "id": 2,
        "date": 1685308916,
        "title": "pick up kids"
    },
    {
        "id": 3,
        "date": 1685309090,
        "title": "pick up groceries"
    }
]
```

#### Update Record

```bash
# curl -H "content-type: application/json" -H "accept: application/json" -X PUT http://localhost:8090/todolist/entries/2 -d '{"title": "pick up kids after playing","date":1685309236}' 2> /dev/null | python3.8 -m json.tool
[
    {
        "id": 1,
        "date": 1685308792,
        "title": "date with wife"
    },
    {
        "id": 2,
        "date": 1685308916,
        "title": "pick up kids after playing"
    },
    {
        "id": 3,
        "date": 1685309090,
        "title": "pick up groceries"
    }
]
```

#### Fetch Records

```bash
# curl -H "content-type: application/json" -H "accept: application/json" -X GET http://localhost:8090/todolist/entries 2> /dev/null | python3.8 -m json.tool
[
    {
        "id": 1,
        "date": 1685308792,
        "title": "date with wife"
    },
    {
        "id": 2,
        "date": 1685308916,
        "title": "pick up kids after playing"
    },
    {
        "id": 3,
        "date": 1685309090,
        "title": "pick up groceries"
    }
]
```

#### Delete Record

```bash
# curl -H "content-type: application/json" -H "accept: application/json" -X DELETE http://localhost:8090/todolist/entries/2 2> /dev/null | python3.8 -m json.tool
[
    {
        "id": 1,
        "date": 1685308792,
        "title": "date with wife"
    },
    {
        "id": 3,
        "date": 1685309090,
        "title": "pick up groceries"
    }
]
```