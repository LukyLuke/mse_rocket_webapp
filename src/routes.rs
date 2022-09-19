
use diesel::prelude::*;

use rocket_contrib::json::Json;
use crate::orm::get_connection;

use crate::dto;
use crate::schema::*;

#[get("/users")]
pub fn users() -> Json<Vec<dto::User>> {
	let users: Vec<dto::User> = users::table
		.select(users::all_columns)
		.load::<dto::User>(&mut get_connection())
		.expect(format!("Error while loading users from Database, so sorry...").as_str());

	Json(users)
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
