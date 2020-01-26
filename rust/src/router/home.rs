use rocket::http::Status;
use rocket::response::NamedFile;
use std::path::Path;

#[get("/")]
pub fn index() -> Result<NamedFile, Status> {
	let path = Path::new("../static/").join("index.html");
	NamedFile::open(&path).map_err(|_| Status::NotFound)
}

#[get("/<file>")]
pub fn static_files(file: String) -> Result<NamedFile, Status> {
	let path = Path::new("../static/").join(file);
	NamedFile::open(&path).map_err(|_| Status::NotFound)
}
