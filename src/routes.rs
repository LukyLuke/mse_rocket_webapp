use diesel::prelude::*;

use rocket::request::{Form, FromForm};
use rocket_csrf::CsrfToken;
use rocket::response::{Flash, Redirect};

use rocket_contrib::json::Json;
use crate::orm::get_connection;

use crate::dto;
use crate::schema::*;

fn not_found(req: &rocket::Request) -> String {
	format!("Something bad happened! Did you really mean '{}'?", req.uri())
}

#[get("/users")]
pub fn users() -> Json<Vec<dto::User>> {
	let users: Vec<dto::User> = users::table
		.select(users::all_columns)
		.load::<dto::User>(&mut get_connection())
		.expect(format!("Error while loading users from Database, so sorry...").as_str());

	Json(users)
}

#[derive(FromForm)]
pub struct LoginForm {
	csrf_token: String,
	username: String,
	password: String,
}

#[post("/login", data = "<form>")]
pub fn login(csrf_token: CsrfToken, form: Form<LoginForm>) -> Flash<Redirect> {
	if let Err(_) = csrf_token.verify(&form.csrf_token) {
		return Flash::error(Redirect::to("/"), "invalid authenticity token");
	}
	Flash::success(
		Redirect::to("/api/users"),
		format!("Login succeeded: {:#?}", form.username),
	)
}

#[post("/user/insert", format = "application/json", data = "<new_user>")]
pub fn user_insert(new_user: Json<dto::NewUser>) -> Json<Vec<dto::User>> {
	diesel::insert_into(users::table)
		.values(&new_user.into_inner())
		.execute(&mut get_connection())
		.expect(format!("Could not insert the new User").as_str());

	user_get(0)
}

#[get("/user/<id>")]
pub fn user_get(id: i32) -> Json<Vec<dto::User>> {
	let user: Vec<dto::User> = users::table
		.filter(users::id.eq(id))
		.limit(1)
		.load(&mut get_connection())
		.expect(format!("Unable to load user with ID {}", id).as_str());

	Json(user)
}
