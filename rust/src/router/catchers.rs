use rocket::response::content;

#[catch(404)]
pub fn catch_404(req: &rocket::Request) -> content::Html<String> {
	content::Html(format!(
		"<p>Sorry, but '{}' is not a valid path!</p>
      <p>Try visiting /hello/&lt;name&gt;/&lt;age&gt; instead.</p>",
		req.uri()
	))
}

#[catch(500)]
pub fn catch_500() -> content::Html<String> {
	content::Html(format!("<p>Internal Server Error. :(</p>"))
}
