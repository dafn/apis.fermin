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

	rocket::ignite()
		.mount("/", routes![webapp::index, webapp::static_files])
		.mount(
			"/api/notes",
			routes![api::notes::get_by_id, api::notes::get_all, api::notes::post],
		)
		.register(catchers![catcher::catch_404, catcher::catch_500])
		.launch();
}
