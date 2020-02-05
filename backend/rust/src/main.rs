#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

extern crate chrono;
extern crate dotenv;
extern crate rustc_serialize;

mod db;
mod router;

use dotenv::dotenv;
use rocket::error::LaunchError;
use router::{api::notes, catcher, webapp};

fn main() {
	dotenv().ok();

	let error: LaunchError = rocket::ignite()
		.mount("/", routes![webapp::index, webapp::static_files])
		.mount(
			"/api/notes",
			routes![notes::get_by_id, notes::get_all, notes::post],
		)
		.register(catchers![catcher::catch_404, catcher::catch_500])
		.launch();

	println!("Whoops! Rocket didn't launch!");
	println!("This went wrong: {}", error);
}
