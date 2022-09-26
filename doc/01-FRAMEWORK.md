## Framework Overview

* [**rust**](https://www.rust-lang.org/learn) is somehow like the modern way for C/C++ in a Unix way of programming: There is no silver bullet but many small different puzzles. Rust is a systems programming language focused on safety, speed, and concurrency.
* [**The main Principle**](https://richard.dallaway.com/2020/05/04/rust-principles.html) in rust is **immutabilty** what means everything you want to change you have to mark as ***mut***. Each variable has it's **owner** which cannot be shared at the same time and will be lost after borrowing it to an other owner, even when borrowed as a (***&***) reference.
* [**Diesel**](https://docs.diesel.rs/2.0.x/diesel/index.html) is an ORM-Framework for rust
* [**Rocket**](https://rocket.rs/) is a web framework for rust
* [**Cargo**](https://doc.rust-lang.org/cargo/) is the main package manager for rust
* [**Crates**](https://crates.io/) are the packages/Libraries used by a rust application and managed by cargo
