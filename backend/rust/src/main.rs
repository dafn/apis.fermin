#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

extern crate dotenv;
extern crate rustc_serialize;

mod db;
mod router;

use dotenv::dotenv;
use rocket::error::LaunchError;
use router::{api::notes, catcher, webapp};

use std::env;

fn main() {
	dotenv().ok();

	let db_connection_pool = db::init_connection_pool(env::var("DATABASE_URL").expect("Could not find 'DATABASE_URL' in env"));

	let error: LaunchError = rocket::ignite()
		.manage(db_connection_pool)
		.mount("/", routes![webapp::index, webapp::static_files])
		.mount(
			"/api/notes",
			routes![notes::get_all, notes::get_by_id, notes::post, notes::put, notes::delete],
		)
		.register(catchers![catcher::catch_404, catcher::catch_500])
		.launch();

	println!("Whoops! Rocket didn't launch!");
	println!("This went wrong: {}", error);
}
