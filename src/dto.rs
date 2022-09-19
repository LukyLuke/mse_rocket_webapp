use crate::schema::*;

use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct User {
	pub id: i32,
	pub name: Option<String>,
	pub surname: Option<String>,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
	pub name: &'a str,
	pub surname: &'a str,
}
