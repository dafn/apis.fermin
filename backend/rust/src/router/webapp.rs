use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::{Path, PathBuf};

#[get("/")]
pub async fn index() -> Result<NamedFile> {
	let path = Path::new("../../frontend/dist/").join("index.html");
	Ok(NamedFile::open(path)?)
}

#[get("/{filename:.*}")]
pub async fn static_files(req: HttpRequest) -> Result<NamedFile> {
	let filename: PathBuf = req.match_info().query("filename").parse().unwrap();
	let path = Path::new("../../frontend/dist/").join(filename);
	Ok(NamedFile::open(path)?)
}
