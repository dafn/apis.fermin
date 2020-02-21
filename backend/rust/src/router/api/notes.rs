use crate::db::models::notes::Note;
use crate::db::DBPool;

use actix_web::{error, web, Error, HttpResponse, http::StatusCode};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NewNoteWithoutLifetime {
  content: String
}

#[get("/")]
pub async fn get_all(db: DBPool) -> Result<HttpResponse, Error> {
  match Note::get_all(&db.get().unwrap()) {
    Ok(all_notes) => Ok(
      HttpResponse::Ok().json(&all_notes)
    ),
    Err(_) => Err(error::ErrorNotFound("File Not Found")),
  }
}

#[get("/{id}")]
pub async fn get_by_id(id: web::Path<i32>, db: DBPool) -> Result<HttpResponse, Error> {
  match Note::get_by_id(&db.get().unwrap(), &id) {
    Ok(note) => Ok(
      HttpResponse::Ok().json(&note)
    ),
    Err(_) => Err(error::ErrorNotFound("File Not Found")),
  }
}

#[post("/")]
pub async fn post(new_note: web::Json<NewNoteWithoutLifetime>, db: DBPool) -> Result<HttpResponse, Error> {
  match Note::post(&db.get().unwrap(), &new_note.content) {
    Ok(_) => Ok(HttpResponse::new(StatusCode::CREATED)),
    Err(_) => Err(error::ErrorNotFound("File Not Found")),
  }
}

/*
#[put("/<id>", data = "<content>")]
pub fn put(db_connection: DbConnection, id: i32, content: String) -> Status {
  match Note::put(&db_connection, &id, &content) {
    Ok(_) => Status::Ok,
    Err(_) => Status::NotFound
  }
}

#[delete("/<id>")]
pub fn delete(db_connection: DbConnection, id: i32) -> Status {
  match Note::delete(&db_connection, &id) {
    Ok(_) => Status::Ok,
    Err(_) => Status::NotFound
  }
}
*/
