use actix_web::web;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub mod models;
pub mod schema;

pub type DBPool = web::Data<Pool<ConnectionManager<PgConnection>>>;

pub fn init_connection(db_url: String) -> Pool<ConnectionManager<PgConnection>> {
  Pool::new(ConnectionManager::<PgConnection>::new(&db_url))
    .expect("Failed initializing connection pool")
}
