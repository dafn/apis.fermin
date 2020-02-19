#![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use]
// extern crate rocket;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate actix_web;

extern crate dotenv;
extern crate rustc_serialize;

mod db;
mod router;

use actix_web::{middleware, web, App, HttpServer};
use router::webapp;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.wrap(middleware::Compress::default())
			.service(webapp::index)
			.service(webapp::static_files)
			// .service(web::scope("/api").service(webapp::index))
	})
	.bind("localhost:8088")?
	.run()
	.await
}
