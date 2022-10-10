use crate::schema::*;

use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct User {
	pub id: i32,
	pub name: Option<String>,
	pub surname: Option<String>,
	pub email: String,
	pub password: String,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
	pub name: &'a str,
	pub surname: &'a str,
	pub email: &'a str,
	pub password: &'a str,
}
