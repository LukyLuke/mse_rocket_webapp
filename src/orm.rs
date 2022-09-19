
use diesel::sqlite::SqliteConnection;
use diesel::Connection;

use dotenvy::dotenv;
use std::env;

pub fn get_connection() -> SqliteConnection {
	dotenv().ok();
	let db_url = env::var("DATABASE_URL")
		.expect("The DATABASE_URL Environment must be set. Create a .env File with DATABASE_URL=./data.sqlite");
	SqliteConnection::establish(db_url.as_str())
		.expect("Unable to connect to the Database. Check the readme for setup.")
}
