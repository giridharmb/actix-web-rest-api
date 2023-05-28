
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