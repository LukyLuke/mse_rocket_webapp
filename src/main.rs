#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod routes;
mod dto;
mod schema;
mod orm;

#[catch(404)]
fn not_found(req: &rocket::Request) -> String {
	format!("Oh no, something went wrong! Did you really mean '{}'?", req.uri())
}

#[get("/")]
fn root() -> &'static str {
	"Hello World, how are you today?"
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
		.launch();
}
