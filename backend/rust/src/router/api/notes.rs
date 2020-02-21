use crate::db::models::notes::Note;
use crate::db::DBPool;

use actix_web::{error, http::StatusCode, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewNoteWithoutLifetime {
  content: String,
}

#[get("/")]
pub async fn get_all(db: DBPool) -> Result<HttpResponse, Error> {
  match Note::get_all(&db.get().unwrap()) {
    Ok(all_notes) => Ok(HttpResponse::Ok().json(&all_notes)),
    Err(_) => Err(error::ErrorNotFound("File Not Found")),
  }
}

#[get("/{id}")]
pub async fn get_by_id(id: web::Path<i32>, db: DBPool) -> Result<HttpResponse, Error> {
  match Note::get_by_id(&db.get().unwrap(), &id) {
    Ok(note) => Ok(HttpResponse::Ok().json(&note)),
    Err(_) => Err(error::ErrorNotFound("File Not Found")),
  }
}

#[post("/")]
pub async fn post(
  new_note: web::Json<NewNoteWithoutLifetime>,
  db: DBPool,
) -> Result<HttpResponse, Error> {
  match Note::post(&db.get().unwrap(), &new_note.content) {
    Ok(_) => Ok(HttpResponse::new(StatusCode::CREATED)),
    Err(_) => Err(error::ErrorNotFound("File Not Found")),
  }
}

#[put("/{id}")]
pub async fn put(
  id: web::Path<i32>,
  updated_note: web::Json<NewNoteWithoutLifetime>,
  db: DBPool,
) -> Result<HttpResponse, Error> {
  match Note::put(&db.get().unwrap(), &id, &updated_note.content) {
    Ok(_) => Ok(HttpResponse::new(StatusCode::OK)),
    Err(_) => Err(error::ErrorNotFound("File Not Found")),
  }
}

#[delete("/{id}")]
pub async fn delete(id: web::Path<i32>, db: DBPool) -> Result<HttpResponse, Error> {
  match Note::delete(&db.get().unwrap(), &id) {
    Ok(_) => Ok(HttpResponse::new(StatusCode::OK)),
    Err(_) => Err(error::ErrorNotFound("File Not Found")),
  }
}
