#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

extern crate chrono;
extern crate dotenv;

mod db;
mod router;

use dotenv::dotenv;
use router::{api, catcher, webapp};

fn main() {
	dotenv().ok();

	let connection = db::connect();

	db::get_notes(&connection);

	db::post(&connection, &"test");

	rocket::ignite()
		.mount("/", routes![webapp::index, webapp::static_files])
		.mount(
			"/api/notes",
			routes![api::notes::get_by_id, api::notes::get_all],
		)
		.register(catchers![catcher::catch_404, catcher::catch_500])
		.launch();
}
