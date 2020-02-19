/*
use crate::db::DbConnection;
use crate::db::models::notes::Note;

use actix_web::HttpResponse;

use rustc_serialize::json;

#[get("/")]
pub fn get_all(db_connection: DbConnection) -> HttpResponse {
  match Note::get_all(&db_connection) {
    Ok(all_notes) => Ok(Json(json::encode(&all_notes).unwrap())),
    Err(_) => Err(Status::InternalServerError)
  }
}
#[get("/<id>")]
pub fn get_by_id(db_connection: DbConnection, id: i32) -> Result<Json<String>, Status> {
  match Note::get_by_id(&db_connection, &id) {
    Ok(note) => Ok(Json(json::encode(&note).unwrap())),
    Err(_) => Err(Status::NotFound)
  }
}

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