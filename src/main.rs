#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;

mod routes;
mod dto;
mod schema;
mod orm;

#[catch(404)]
fn not_found(req: &rocket::Request) -> String {
	format!("Oh no, something went wrong! Did you really mean '{}'?", req.uri())
}

#[get("/")]
fn root() -> Template {
	let user = dto::User {
		id: -1,
		name: Some(String::from("Please")),
		surname: Some(String::from("Login")),
	};
	Template::render("home", user)
}

fn main() {
	rocket::ignite()
		.register(catchers![not_found])
		.mount("/", routes![ root ])
		.mount("/api", routes![
			routes::users,
			routes::user_insert,
			routes::user_get,
		])
		.attach(Template::fairing())
		.launch();
}
