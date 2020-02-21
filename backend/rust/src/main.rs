#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate actix_web;

extern crate dotenv;
extern crate rustc_serialize;

mod db;
mod router;

use actix_web::{middleware, web, App, HttpServer};

use dotenv::dotenv;
use router::{api, webapp};

use std::env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();

	env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();

	HttpServer::new(|| {
		App::new()
			.data(db::init_connection(
				env::var("DATABASE_URL").expect("Could not find 'DATABASE_URL' in env"),
			))
			.wrap(middleware::Compress::default())
			.wrap(middleware::Logger::new(" %s | %U"))
			.service(webapp::index)
			.service(
				web::scope("/api/notes")
					.service(api::notes::get_all)
					.service(api::notes::get_by_id)
					.service(api::notes::post)
					.service(api::notes::put)
					.service(api::notes::delete),
			)
			.service(webapp::static_files)
	})
	.bind("localhost:8088")?
	.run()
	.await
}
