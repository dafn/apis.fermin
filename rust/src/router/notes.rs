#[get("/")]
pub fn get_all() -> &'static str {
  "notes /"
}

#[get("/<id>")]
pub fn get_by_id(id: usize) -> String {
  format!("Note with id {}", id)
}
