use diesel::prelude::*;

use rocket::request::{Form, FromForm};
use rocket_csrf::CsrfToken;
use rocket::response::{Flash, Redirect};

use rocket_contrib::json::Json;
use crate::orm::get_connection;

use crate::dto;
use crate::schema::*;

use bcrypt::{DEFAULT_COST, hash, verify};

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
	email: String,
	password: String,
}

#[derive(FromForm)]
pub struct SignupForm {
	csrf_token: String,
	email: String,
	password: String,
	password2: String,
	firstname: String,
	lastname: String,
}

#[post("/login", data = "<form>")]
pub fn login(csrf_token: CsrfToken, form: Form<LoginForm>) -> Flash<Redirect> {
	if let Err(_) = csrf_token.verify(&form.csrf_token) {
		return Flash::error(Redirect::to("/csrf-token-invalid"), "invalid authenticity token");
	}
	// search for user with email from form in database
	let user: Vec<dto::User> = users::table
		.filter(users::email.eq(form.email.to_string()))
		.limit(1)
		.load(&mut get_connection())
		.expect(format!("Unable to load user with email {}", form.email).as_str());
	// check against stored password in database
	let password_check_result = verify(form.password.to_string(), &user[0].password);
	if ! password_check_result.unwrap() {
		return Flash::error(Redirect::to("/password-invalid"), "invalid password");
	}
	Flash::success(
		Redirect::to("/api/users"),
		format!("Login succeeded: {:#?}", form.email),
	)
}

#[post("/signup", data = "<form>")]
pub fn signup(csrf_token: CsrfToken, form: Form<SignupForm>) -> Redirect {
	// check csrf_token is valid
	if let Err(_) = csrf_token.verify(&form.csrf_token) {
		return Redirect::to("/csrf-token-invalid");
	}
	// Check if the passwords match
	if form.password != form.password2 {
		return Redirect::to("/error");
	}
	// Hash the password for storage
	let password = hash(&form.password.to_string(), DEFAULT_COST);

	// build the struct from the form
	let new_user = dto::NewUser {
		email: &form.email.to_string(),
		password: &password.unwrap(),
		surname: &form.firstname.to_string(),
		name: &form.lastname.to_string(),
	};

	// insert the user into the database
	diesel::insert_into(users::table)
		.values(&new_user)
		.execute(&mut get_connection())
		.expect(format!("Could not insert the new User").as_str());
	return Redirect::to("/");
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
