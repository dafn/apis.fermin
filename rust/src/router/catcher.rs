/*
use rocket::response::content::Html;
use rocket::Request;

#[catch(404)]
pub fn catch_404(req: &Request) -> Html<String> {
	Html(format!(
		"<p>404: Not Found</p><br><p>path: {} </p>",
		req.uri()
	))
}

#[catch(500)]
pub fn catch_500() -> Html<String> {
	Html(format!("<p>Internal Server Error. :(</p>"))
}
*/