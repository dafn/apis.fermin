use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use std::ops::Deref;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

pub mod models;
pub mod schema;

type DbConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_connection_pool(db_url: String) -> DbConnectionPool {
  Pool::new(ConnectionManager::<PgConnection>::new(db_url))
    .expect("Failed initializing connection pool")
}

pub struct DbConnection(pub PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConnection {
  type Error = ();

  fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConnection, ()> {
    match request.guard::<State<DbConnectionPool>>()?.get() {
      Ok(connection) => Outcome::Success(DbConnection(connection)),
      Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
    }
  }
}

impl Deref for DbConnection {
  type Target = PgConnection;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
