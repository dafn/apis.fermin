use crate::db::models::notes::Note;
use crate::db::DbConnection;

use actix_web::{error, web, Error, HttpResponse};

use rustc_serialize::json;

#[get("/")]
pub async fn get_all(db: DbConnection) -> Result<HttpResponse, Error> {
  match Note::get_all(&db.get().unwrap()) {
    Ok(all_notes) => Ok(
      HttpResponse::Ok()
        .content_type("application/json")
        .body(json::encode(&all_notes).unwrap()),
    ),
    Err(_) => Err(error::ErrorNotFound("File Not Found")),
  }
}

#[get("/{id}")]
pub async fn get_by_id(id: web::Path<i32>, db: DbConnection) -> Result<HttpResponse, Error> {
  match Note::get_by_id(&db.get().unwrap(), &id) {
    Ok(note) => Ok(
      HttpResponse::Ok()
        .content_type("application/json")
        .body(json::encode(&note).unwrap()),
    ),
    Err(_) => Err(error::ErrorNotFound("File Not Found")),
  }
}

/*
#[post("/", data = "<content>")]
pub fn post(db_connection: DbConnection, content: String) -> Status {
  match Note::post(&db_connection, &content) {
    Ok(_) => Status::Created,
    Err(_) => Status::InternalServerError
  }
}

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
