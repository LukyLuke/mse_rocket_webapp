# mse_rocket_webapp

MSE ITSecurity Webapp Project

## Run the app

```
$ cargo run
...
Rocket has launched from http://localhost:8000
...
```

Point your browser to http://localhost:8000


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
