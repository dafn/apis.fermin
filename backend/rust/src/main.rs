#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod router;

use router::{catchers, home, notes};

fn main() {
	rocket::ignite()
		.mount("/", routes![home::index, home::static_files])
		.mount("/notes", routes![notes::get_by_id, notes::get_all])
		.register(catchers![catchers::catch_404, catchers::catch_500])
		.launch();
}
