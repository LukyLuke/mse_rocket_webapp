#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod routes;
mod dto;
mod schema;
mod orm;

#[get("/")]
fn root() -> &'static str {
	"Hello World, how are you today?"
}

fn main() {
	rocket::ignite()
		.mount("/", routes![
			root,
			routes::users,
			routes::user_insert,
			routes::user_get,
		]).launch();
}
