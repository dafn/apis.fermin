use chrono::NaiveDate;
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

pub fn get_notes(connection: &PgConnection) {
  let results = notes
    .limit(50)
    .load::<Note>(connection)
    .expect("Error loading notes");

  println!("Displaying {} notes", results.len());

  for note in results {
    println!("{}", note.id);
    println!("{}", note.content);
  }
}

pub fn post<'a>(connection: &PgConnection, new_content: &str) {
  use schema::notes;

  let new_note = NewNote {
    content: new_content,
  };

  diesel::insert_into(notes::table)
    .values(&new_note)
    .execute(connection)
    .expect("Error saving new note");
}
