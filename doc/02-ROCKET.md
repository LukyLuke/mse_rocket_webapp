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


## Cross Site Scripting (XSS)

Provided you are using the builtin templates there is XSS protection.

### Prevent XSS

Rocket offers builtin template rendering that supports the popular handlebars
or tera templating languages.

For template rendering we are using the handlebars which by default escapes the
characters `"&<>` using their HTML entities. [Handlebars
Escaping](https://api.rocket.rs/master/rocket_dyn_templates/handlebars/index.html#escaping)

This can further be improved by providing your own escaping function or you can
shoot yourself in the foot by explicitly using the provided no_escape()
function.

Another way would be to implement a Content Security Policy (CSP) although
Rocket does not natively support this meaning if you want to do it, you would
have to implement it by hand.

[Open Issue for native support of CSP](https://github.com/SergioBenitez/Rocket/issues/264)

## CrossSite Request Forgery (CSRF)

There is no automatic prevention for CSRF.

### Preventing CSRF

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

## User+Password Management

There is no builtin User Management.

### How to do User Management

Some other crates add support for this. There is a [long standing issue from
2016](https://github.com/SergioBenitez/Rocket/issues/8) that discusses a
builtin solution.

Adding something like
[rocket_auth](https://docs.rs/rocket_auth/latest/rocket_auth/) would work but
it brings its own database handler which is not based on diesel but will rather
write to a database itself.

What we ended up doing is implementing the user management ourselves and
hashing the passwords ourselves. We settled on
[Bcrypt](https://github.com/Keats/rust-bcrypt) with a work factor of 12.
Changing away from that is a lot of work since the existing passwords would
need to be migrated as well.

## Session Management

There is no Session Management built in Rocket although there are middlewares
available to do that.

[rocket_auth](https://docs.rs/rocket_auth/latest/rocket_auth/) offers this
functionality but uses its own database library that does not use diesel.

The builtin mechanism that tries to replace Sessions are encrypted cookies. So
any value you would want to save in a session you just write into a cookie that
is encrypted. Please note that it is imperative that you set the secret_key
config option otherwise the key used for encrypting the cookies is regenereated
on restart of the application.

### How to do Session Management

There is not really a session concept built in. You can implement it yourself
or use the encrypted cookies.

See https://api.rocket.rs/v0.4/rocket/http/enum.Cookies.html
