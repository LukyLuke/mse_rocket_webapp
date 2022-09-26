# mse_rocket_webapp

MSE ITSecurity Webapp Project

See (Documentation)[doc/README.md] for the different libraries and explanations.

## Run the app

```
$ cargo run
...
Rocket has launched from http://localhost:8000
...
```

Point your browser to http://localhost:8000

## Insert a new user

```
curl -X POST -H "Content-Type: application/json" --data '{"name": "Frank", "surname": "Nord"}' http://localhost:8000/user/insert
```

# Setup your Environment

## Using RUST

Use rustup for having a nightly build of rust which is needed for rocket. Rustup is for having rust uptodate.

```
$ rustup default nightly
$ rustup update
```

## Using Cargo

Cargo is the Package manager: https://crates.io/

```
$ cargo build
$ cargo run
$ cargo build
$ cargo build --release
```

## Using diesel ORM

```
$ cargo install diesel_cli --no-default-features --features sqlite
$ $HOME/.cargo/bin/diesel migration run
$ $HOME/.cargo/bin/diesel migration redo
```


## Crosscompile

Check the different targets here: https://rust-lang.github.io/rustup-components-history

### For Windows:

```
$ rustup target add nightly-x86_64-pc-windows-gnu
$ cargo build -r --target nightly-x86_64-pc-windows-gnu
```

### For Linux:

```
$ rustup target add nightly-x86_64-unknown-linux-gnu
$ cargo build -r --target nightly-x86_64-unknown-linux-gnu
```

### For Mac OSX:

```
$ rustup target add nightly-x86_64-apple-darwin
$ cargo build -r --target nightly-x86_64-apple-darwin
```
