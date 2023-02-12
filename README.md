# practice-rust



## Lesson 1
* Gone through the [rust-lang first section](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
* Used `rustc hello.rs` and `./hello` to compile and execute the `hello_world` example
* Used `cargo new`, `cargo build`, and `cargo run` to create the `hello_cargo` example
* Added `/**/target/` in root directory's `.gitignore` to avoid committing binary files (this will only work for projects created via `cargo`, otherwise the binary file won't be under `target` folder)

## Lesson 2
* Intro to many Rust features by creating the `guessing_game`
    * `let`: define immutable variable, and to make it mutable, do `let mut`
    * Shadowing: declare a new variable with the same name as a previous variable
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
* Some stuff I already find quite awesome 👀:
    * `cargo` to Rust feels more integrated than `pip` to Python
    * Giving warning on what should be handled to prevent unexpected behaviors or unmet expectations
    * CLEAN - `loop` and `match` combo - 🤩
    * trait?? what's trait?? is this part of type safe mechanism
    * indeed compiled language makes code distribution safer and faster in some cases - dynamic languages are stronger for experimental uses

## Lesson 3
* Immutable variable
    * ex. `let x = 5`
* Mutable variable
    * ex. `let mut x = 5`
* Constant
    * always immutable
    * use `const` keyword
    * naming convention - always uppercase with underscore between words
    * type of value MUST be annotated
    * can be defined in any scope
    * MUST be set to a constant expression, not something computed upon runtime
    * ex. `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`
* Shadowing
    * declare a new variable with the same name as a previous variable
    * has to use `let` keyword when defining the variable of the same name again
    * DIFFERENT from making a variable `mut`
        * can perform transformations on an immutable variable and it remains immutable
        * can change the data type of the immutable variable
    * Benefits:
        * avoid creating "x_str", "x_num" and just use "x"
    * ex. this works 🟢
        ```
        fn main() {
            let spaces = "   ";
            let spaces = spaces.len();
        }
        ```
        this does not work 🔴
        ```
        fn main() {
            let mut spaces = "   ";
            spaces = spaces.len();
        }

        --> returns "mismatched types" error
        ```
* Data Type: Rust is a statically type language, meaning if it doesn't know the variable type and many types are possible, it will throw a compiler error.
    * Annotated after a variable
        * ex. `let x: f32 = 1.0;`
              `let guess: u32 = "42".parse().expect("Not a number!");`
    * Rust is zero-indexed
    * Scalar
        * Integer
            * Unsigned
                * 0 to 2n - 1
                * ex. an u8 can store numbers from 0 to 28 - 1, which equals 0 to 255
            * Signed
                * -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses
                * ex. an i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127
            * Integer division truncates toward zero to the nearest integer
                * ex. -5 / 3 = -1
        * Floating-point number
            * Numbers with decimal points
            * Always signed
            * f32 - 32 bits, f64 - 64 bits
        * Boolean
            * true / false
        * Character
            * `char` literals are in single quotes, as opposed to string interals that are in double quotes
            * 4 bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII
    * Compound
        * Tuple
            * Fixed length
            * Comma-separated within ()
            * Types of different values do NOT have to be the same
            * Destructuring: use pattern matching to get a value within a tuple
                * ex. 
                ```
                let tup = (500, 6.4, 1);

                let (x, y, z) = tup;

                --> y = 6.4
                ```
            * Access tuple elements through a period (`.`)
                * ex.
                ```
                let x: (i32, f64, u8) = (500, 6.4, 1);

                let five_hundred = x.0;

                let six_point_four = x.1;

                let one = x.2;
                ```
            * "Unit": means an empty tuple, like `()`
        * Array
            * Fixed length
            * Comma-separated within []
            * Types of different values HAVE to be the same
            * Useful when you want your data allocated on the stack rather than the heap or when you want to ensure you always have a fixed number of elements
            * Write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array
                * ex.
                ```
                let a: [i32; 5] = [1, 2, 3, 4, 5];
                ```
            * Can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets
                * ex.
                ```
                let a = [3; 5];

                --> the same as let a = [3, 3, 3, 3, 3];
                ```
            * Access array elements through indexing
                * ex.
                ```
                let a = [1, 2, 3, 4, 5];

                let first = a[0];
                let second = a[1];
                ```
            * "Out of bound": the program "panics" if the index is larger than the length of the array - this is a "Runtime error" because the compiler cannot know what the user will input until runtime
            * First example of Rust's memory safety principle: the program does not waste time on continuing an invalid memory access
* Function
    * Parameters
        * MUST declare the type of the parameter
        * Separate multiple parameters by comma
    * Expression VS Statements
        * Statements
            * Instructions that perform some action
            * Do not return a value
            * Ends with a semicolon (`;`)
            * `let y = 6;`, a function definition
        * Expressions
            * Evaluate to a resultant value
            * Does NOT end with a semicolon (`;`)
            * Calling a function, a macro, a new scope block created with `{}`
    * Return values
        * We do not name the returned value
        * Declare type via (`->`)
        * Return value ~= value of the last expression in the function's body (implicit)
            * Can also return early by using `return` keyword and specify a value
        * ex. this works 🟢
            ```
            fn five() -> i32 {
                5
            }

            fn main() {
                let x = five();

                println!("The value of x is: {x}");
            }
            ```
            5 is an expression

            ```
            fn main() {
                let x = plus_one(5);

                println!("The value of x is: {x}");
            }

            fn plus_one(x: i32) -> i32 {
                x + 1
            }
            ```
            x + 1 is an expression

            If we added `;` to the end of x + 1, this function will fail with a mismatched type error because it's expecting a `i32` type instead of a `unit` `()` type.