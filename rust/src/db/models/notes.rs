use chrono::NaiveDateTime;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::*;

use crate::db::schema::notes as notes_schema;
use crate::db::schema::notes::dsl::notes;

use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Note {
  pub id: i32,
  pub content: String,
  pub created: NaiveDateTime,
  pub last_modified: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "notes_schema"]
pub struct NewNote<'a> {
  pub content: &'a str,
}

impl Note {
  pub fn get_all(connection: &PgConnection) -> Result<Vec<Note>, Error> {
    notes
      .order(notes_schema::id.desc())
      .load::<Note>(connection)
  }

  pub fn get_by_id<'a>(connection: &PgConnection, _id: &'a i32) -> Result<Note, Error> {
    notes.find(_id).first::<Note>(connection)
  }

  pub fn post<'a>(connection: &PgConnection, _content: &'a str) -> Result<usize, Error> {
    diesel::insert_into(notes_schema::table)
      .values(&NewNote { content: _content })
      .execute(connection)
  }

  pub fn put<'a, 'b>(
    connection: &PgConnection,
    _id: &'a i32,
    _content: &'b str,
  ) -> Result<Note, Error> {
    diesel::update(notes.find(_id))
      .set((
        notes_schema::content.eq(_content),
        notes_schema::last_modified.eq(chrono::offset::Utc::now().naive_local()),
      ))
      .get_result::<Note>(connection)
  }

  pub fn delete<'a>(connection: &PgConnection, _id: &'a i32) -> Result<usize, Error> {
    diesel::delete(notes.find(_id)).execute(connection)
  }
}
