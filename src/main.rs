#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;
use serde::{Serialize};
use rocket_csrf::CsrfToken;

mod routes;
mod dto;
mod schema;
mod orm;

#[catch(404)]
fn not_found(req: &rocket::Request) -> String {
	format!("Oh no, something went wrong! Did you really mean '{}'?", req.uri())
}

#[derive(Serialize)]
pub struct Renderer {
	pub xss_test: String,
	pub csrf_token: String,
}

#[get("/")]
fn root(csrf_token: CsrfToken) -> Template {
	let authenticity_token: String = csrf_token.authenticity_token();
	let test = Renderer{
		xss_test: String::from("</h1><script>alert(\"No...\");</script>"),
		csrf_token: authenticity_token,
	};
	Template::render("home", test)
}

fn main() {
	rocket::ignite()
		.register(catchers![not_found])
		.mount("/", routes![ root ])
		.mount("/api", routes![
			routes::users,
			routes::user_insert,
			routes::user_get,
			routes::login,
		])
		.attach(rocket_csrf::Fairing::default())
		.attach(Template::fairing())
		.launch();
}
