use diesel::pg::PgConnection;
use diesel::prelude::*;
use model::{NewNote, Note};
use schema::notes::dsl::*;
use std::env;

pub mod model;
pub mod schema;

pub fn connect() -> PgConnection {
  let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn get_notes(connection: &PgConnection) -> Vec<Note> {
  let result = notes
    .limit(5)
    .load::<Note>(connection)
    .expect("Error loading notes");

  println!("Displaying {} notes", result.len());

  for note in &result {
    println!("{}", note.id);
  }

  result
}

pub fn post_note<'a>(connection: &PgConnection, new_content: &'a str) -> usize {
  use schema::notes;

  let new_note = NewNote {
    content: new_content,
  };

  let result = diesel::insert_into(notes::table)
    .values(&new_note)
    .execute(connection)
    .expect("Error saving new note");

  result
}

pub fn get_by_id<'a>(connection: &PgConnection, _id: &'a i32) -> Vec<Note> {
  notes.select((id, content)).filter(id.eq(_id)).limit(1).load(connection).unwrap()
}
