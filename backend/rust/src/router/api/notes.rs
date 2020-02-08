use crate::db::connect;
use crate::db::model::Note;

use rocket::response::status;
use rocket::http::Status;

use rustc_serialize::json;

#[get("/")]
pub fn get_all() -> String {
  json::encode(&Note::get_all(&connect())).unwrap()
}

#[get("/<id>")]
pub fn get_by_id(id: i32) -> Result<String, status::NotFound<String>> {
  match Note::get_by_id(&connect(), &id) {
    Ok(note) => Ok(json::encode(&note).unwrap()),
    Err(_) => Err(status::NotFound(format!("Could not find note with id {}", &id)))
  }
}

#[post("/<content>")]
pub fn post(content: String) -> Status {
  match Note::post(&connect(), &content) {
    true => Status::Created,
    false => Status::InternalServerError
  }
}

#[put("/<id>/<content>")]
pub fn put(id: i32, content: String) -> Status {
  match Note::put(&connect(), &id, &content) {
    true => Status::Ok,
    false => Status::NotFound
  }
}

#[delete("/<id>")]
pub fn delete(id: i32) -> Status {
  match Note::delete(&connect(), &id) {
    true => Status::Ok,
    false => Status::NotFound
  }
}
