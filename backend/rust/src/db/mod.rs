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
