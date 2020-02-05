use crate::db;
use crate::db::connect;
use crate::db::model::Note;

use rustc_serialize::json;

#[get("/")]
pub fn get_all() -> String {
  #[derive(RustcDecodable, RustcEncodable)]
  let all_notes: Vec<Note> = db::get_notes(&connect());

  json::encode(&all_notes).unwrap()
}

#[get("/<id>")]
pub fn get_by_id(id: usize) -> String {
  format!("Note with id '{}'", id)
}

#[post("/<content>")]
pub fn post(content: String) {
  db::post_note(&connect(), &content);
}

#[delete("/<id>")]
pub fn delete(id: usize) {
  // todo: delete
}
