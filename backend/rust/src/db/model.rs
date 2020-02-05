use super::schema::notes;
use rustc_serialize::json;

#[derive(Queryable, RustcDecodable, RustcEncodable)]
pub struct Note {
    pub id: i32,
    pub content: String,
}

#[derive(Insertable)]
#[table_name="notes"]
pub struct NewNote<'a> {
  pub content: &'a str
}
