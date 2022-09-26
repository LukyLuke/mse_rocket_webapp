## Diesel ORM-Framework

### Minimal example: Diesel

#### Prerequisite

The *diesel* binary is needed, installable by cargo manually. This creates the schema as rust macros.

`$ cargo install diesel_cli --no-default-features --features sqlite|mysql|pg`

#### SQL: migrations/TIMESTAMP_xy/

* *up.sql*
```
CREATE TABLE users (
	id INTEGER PRIMARY KEY NOT NULL,
	name NVARCHAR(250),
	surname NVARCHAR(250)
);
```

* *down.sql*
```
DROP TABLE users;
```

#### DTO Representation

* *dto.rs*

```
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
```

#### Generates schema

* Define where the schema should be stored in *diesel.toml*
```
[print_schema]
file = "src/schema.rs"
```

* Build and rebuilt with the command: `diesel migration run`
* The schema the is stored in *src/schema.rs`
```
// @generated automatically by Diesel CLI.
diesel::table! {
    users (id) {
        id -> Integer,
        name -> Nullable<Text>,
        surname -> Nullable<Text>,
    }
}
```

#### Establish a Connection

* *.env* file which has an environment variable DATABASE_URL inside. Holds the username, password, server and schema/db. Should be `0600` by the user running the application.
* *db_connect.rs*
```
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
```

#### Querying data

* *query*
```
pub fn user_get(id: i32) -> Json<Vec<dto::User>> {
	let user: Vec<dto::User> = users::table
		.filter(users::id.eq(id))
		.limit(1)
		.load(&mut get_connection())
		.expect(format!("Unable to load user with ID {}", id).as_str());
	Json(user)
}
```

* *insert/update*
```
pub fn user_insert(new_user: Json<dto::NewUser>) -> Json<Vec<dto::User>> {
	diesel::insert_into(users::table)
		.values(&new_user.into_inner())
		.execute(&mut get_connection())
		.expect(format!("Could not insert the new User").as_str());
	user_get(0)
}
```


### Prevent SQLi

#### Finding an object

In [**Diesel**](https://docs.diesel.rs/2.0.x/diesel/index.html) we can only work with Objects. Therefore no SQL can be used in any form (except as DDL for migration). This is all done with the (query_dsl)[https://docs.diesel.rs/2.0.x/diesel/query_dsl/index.html].

* **Finding an Object**
```
let user_id = users
    .filter(name.eq("Max"))
    .filter(surname.like("Muster%"))
    .select(id)
    .first(connection);
```

#### Escaping

There is no need for escaping except you know that for example SqLite is not able to handle `\0` in a text field or so.

Each driver has it's own overridden mechanism to replace unwanted symbols:
* **sqlite** [escapes backticks and encapsulates values in them](https://github.com/diesel-rs/diesel/blob/master/diesel/src/sqlite/query_builder/mod.rs):
```
    fn push_identifier(&mut self, identifier: &str) -> QueryResult<()> {
        self.push_sql("`");
        self.push_sql(&identifier.replace('`', "``"));
        self.push_sql("`");
        Ok(())
    }
```
* **mysql** [escapes backticks and encapsulates values in them](https://github.com/diesel-rs/diesel/blob/master/diesel/src/mysql/query_builder/mod.rs)
```
    fn push_identifier(&mut self, identifier: &str) -> QueryResult<()> {
        self.push_sql("`");
        self.push_sql(&identifier.replace('`', "``"));
        self.push_sql("`");
        Ok(())
    }
```
* **pgsql** [escapes quotes and encapsulates values in doublequotes](https://github.com/diesel-rs/diesel/blob/master/diesel/src/pg/query_builder/mod.rs)
```
    fn push_identifier(&mut self, identifier: &str) -> QueryResult<()> {
        self.push_sql("\"");
        self.push_sql(&identifier.replace('"', "\"\""));
        self.push_sql("\"");
        Ok(())
    }
```
* **mysql** and **sqlite** have unnumbered identifiers `?, ?, ...`
* **pgsql** has numbered identifiers like `$1, $2, ...`
* **pgsql** has some more DQL specific handlers

