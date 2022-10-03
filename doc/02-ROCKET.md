# Rocket Web-Framework

## Minimal example: Rocket

* There is only the *main.rs* file needed
```
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[get("/")]
fn root() -> &'static str {
	"Hello World, how are you today?"
}

fn main() {
	rocket::ignite()
		.mount("/", routes![
			root,
		]).launch();
}
```
* With this in place it can be fired up with cargo
```
$ cargo run
...
Rocket has launched from http://localhost:8000
...
```
* Building it with `cargo build -r` as a release.
* Use the flags `-C target-feature=+crt-static` for static build
* Just have the small binary in a Container and start it up.


## CrossSite-Scription (XSS)

There is no automatic prevention for XSS.

### Prevent XSS

> TODO


## CrossSite Request Forgery (CSRF)

There is no automatic prevention for CSRF.

### Prevent CSRF

> TODO


## User Management

There is no User Management.

### How to do User Management

> TODO


## Session Management

There is no Session Management.

### How to do Session Management

> TODO

