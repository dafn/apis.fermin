use crate::db;
use crate::db::connect;
use crate::db::model::Note;

#[get("/")]
pub fn get_all() -> &'static str {
  "notes /"
}

#[get("/<id>")]
pub fn get_by_id(id: usize) -> String {
  format!("Note with id {}", id)
}

#[post("/<content>")]
pub fn post(content: String) {
  db::post(&connect(), &content);
}

#[delete("/<id>")]
pub fn delete(id: usize) {
  // todo: delete
}