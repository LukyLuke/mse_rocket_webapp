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

Rocket offers builtin template rendering that supports the popular handlebars
or tera templating languages.

TODO figure out if it does escaping or not, the next two paragraph below are
referring to a different library that is NOT built into rocket.

For template rendering we are using [handlebars](https://docs.rs/handlebars/latest/handlebars/) which by default escapes the
characters `"&<>` using their HTML entities.
[Handlebars Escaping](https://api.rocket.rs/master/rocket_dyn_templates/handlebars/index.html#escaping)

This can further be improved by providing your own escaping function or you can
shoot yourself in the foot by explicitly using the provided no_escape()
function.

Another way would be to implement a Content Security Policy (CSP) although
Rocket does not natively support this meaning if you want to do it, you would
have to implement it by hand.

[Open Issue for native support of CSP](https://github.com/SergioBenitez/Rocket/issues/264)

## CrossSite Request Forgery (CSRF)

There is no automatic prevention for CSRF.

### Prevent CSRF

Preventing CSRF is a matter of adding a server signed piece of information that
is added for example when rendering a form template as an additional input
field. Upon submission the server can check the validity of the CSRF token by
verifying the signature.

In Rocket this is not supported natively so you would need to do it yourself.
Loads of foot guns here.

[Open issue](https://github.com/SergioBenitez/Rocket/issues/14)

Using CSRF is possible with
[rocket_csrf](https://github.com/kotovalexarian/rocket_csrf) as a fairing
(rocket term for middleware) although the project page shows this warning:

```
WARNING! The implementation is very simple for now and may not be ready for production.
```

> TODO

## User Management

There is no builtin User Management.

### How to do User Management

Some other crates add support for this. There is a [long standing issue from
2016](https://github.com/SergioBenitez/Rocket/issues/8) that discusses a
builtin solution.

Adding something like
[rocket_auth](https://docs.rs/rocket_auth/latest/rocket_auth/) would work but
it brings its own database handler which is not based on diesel but will rather
write to a database itself.

> TODO

## Session Management

There is no Session Management built in Rocket although there are middlewares
available to do just that.

[rocket_auth](https://docs.rs/rocket_auth/latest/rocket_auth/) offers this
functionality but uses its own database library that does not use diesel.

### How to do Session Management

> TODO
