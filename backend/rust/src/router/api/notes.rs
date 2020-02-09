use crate::db::connect;
use crate::db::model::Note;

use rocket::response::content::Json;
use rocket::http::Status;

use rustc_serialize::json;

#[get("/")]
pub fn get_all() -> Result<Json<String>, Status> {
  match Note::get_all(&connect()) {
    Ok(all_notes) => Ok(Json(json::encode(&all_notes).unwrap())),
    Err(_) => Err(Status::InternalServerError)
  }
}

#[get("/<id>")]
pub fn get_by_id(id: i32) -> Result<Json<String>, Status> {
  match Note::get_by_id(&connect(), &id) {
    Ok(note) => Ok(Json(json::encode(&note).unwrap())),
    Err(_) => Err(Status::NotFound)
  }
}

#[post("/<content>")]
pub fn post(content: String) -> Status {
  match Note::post(&connect(), &content) {
    Ok(_) => Status::Created,
    Err(_) => Status::InternalServerError
  }
}

#[put("/<id>/<content>")]
pub fn put(id: i32, content: String) -> Status {
  match Note::put(&connect(), &id, &content) {
    Ok(_) => Status::Ok,
    Err(_) => Status::NotFound
  }
}

#[delete("/<id>")]
pub fn delete(id: i32) -> Status {
  match Note::delete(&connect(), &id) {
    Ok(_) => Status::Ok,
    Err(_) => Status::NotFound
  }
}
