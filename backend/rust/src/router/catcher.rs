use rocket::response::content;

#[catch(404)]
pub fn catch_404(req: &rocket::Request) -> content::Html<String> {
	content::Html(format!(
		"<p>404: Not Found</p><br><p>path: {} </p>",
		req.uri()
	))
}

#[catch(500)]
pub fn catch_500() -> content::Html<String> {
	content::Html(format!("<p>Internal Server Error. :(</p>"))
}
