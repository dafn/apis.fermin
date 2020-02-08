use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::*;

use super::schema::notes as notes_schema;
use super::schema::notes::dsl::notes;

#[derive(Queryable, RustcDecodable, RustcEncodable)]
pub struct Note {
  pub id: i32,
  pub content: String,
}

#[derive(Insertable)]
#[table_name = "notes_schema"]
pub struct NewNote<'a> {
  pub content: &'a str,
}

impl Note {
  pub fn get_all(connection: &PgConnection) -> Vec<Note> {
    notes
      .limit(5)
      .order(notes_schema::id.desc())
      .load::<Note>(connection)
      .expect("Error loading notes")
  }

  pub fn get_by_id<'a>(connection: &PgConnection, _id: &'a i32) -> Result<Note, Error> {
    notes.find(_id).first::<Note>(connection)
  }

  pub fn post<'a>(connection: &PgConnection, _content: &'a str) -> bool {
    diesel::insert_into(notes_schema::table)
      .values(&NewNote { content: _content })
      .execute(connection)
      .is_ok()
  }

  pub fn put<'a, 'b>(connection: &PgConnection, _id: &'a i32, _content: &'b str) -> bool {
    diesel::update(notes.find(_id))
      .set(notes_schema::content.eq(_content))
      .get_result::<Note>(connection)
      .is_ok()
  }

  pub fn delete<'a>(connection: &PgConnection, _id: &'a i32) -> bool {
    diesel::delete(notes.find(_id)).execute(connection).is_ok()
  }
}
