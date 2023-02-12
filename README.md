# practice-rust

## Lesson 1
* Gone through the [rust-lang first section](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
* Used `rustc hello.rs` and `./hello` to compile and execute the `hello_world` example
* Used `cargo new`, `cargo build`, and `cargo run` to create the `hello_cargo` example
* Added `/**/target/` in root directory's `.gitignore` to avoid committing binary files (this will only work for projects created via `cargo`, otherwise the binary file won't be under `target` folder)

## Lesson 2
* Intro to many Rust features by creating the `guessing_game`
    * `let`: define immutable variable, and to make it mutable, do `let mut`
    * `match`: expression that compares values using each "arm" within `{}`, exits the expression upon a successful match
    * `loop`: an infinite loop that will execute forever unless there's a `break` condition or the program crashes
    * `enum`: a type that can be in multiple possible states
        * `variant`: each possible state
    * `Result`: more on result values expected to be handled, otherwise would throw a warning
    * `fn`: entrypoint of Rust code, defining a function
    * `&`: reference to a variable that does not need to save that data in memory
    * `use`: the equivalent to `import` in Python to make a dependency available for use
    * `::` as **associated function**: used to instantiate a function
       * ex. `let mut guess = String::new();` means "create a mutable variable that is currently bound to a new, empty instance of `String`."
    * `1..=100`: range expression to request a number inclusive between 1 and 100.
* Intro to [crate.io](crate.io) and how Cargo can manage dependency's version updates
* Still unclear 😦:
    * It seems like `::` is very powerful and when used in different context, it could mean
        * using a method from a dependency
        * instantiate an associated function
        * specifying a trait
    * Difference between dependency, macro, and function
    * When to use mutable vs immutable variable
    * `cargo update` didn't seem to update the `rand` package to 0.8.6 even upon `cargo build`
