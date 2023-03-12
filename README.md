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
            * Unsigned (`u32`)
                * 0 to 2n - 1
                * ex. an u8 can store numbers from 0 to 28 - 1, which equals 0 to 255
            * Signed (`i32`)
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
* Control Flow
    * `if`
        * "arm": block of code associated with conditions
        * conditions MUST be boolean because Rust will not automatically try to convert non-Boolean types to a Boolean (like Javascript or Ruby)
        * Rust only executes the block for the first true condition, and once it finds one, it doesn’t even check the rest.
            * general anatomy:
            ```
            if boolean_condition {

            } else if another_boolean_condition {

            } else {

            }
            ```
        * might want to refactor using `match` if there are too many if conditions
        * an expression that can be on the right side of the `let` keyword to assign a variable
            * blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions
            * both the `if` arm and the `else` arm have to be the same data type
            * ex.
            ```
            let number = if condition { 5 } else { 6 };
            ```
    * loops
        * `loop`: infinite loop until `break` keyword or manually crashing the program
            * `continue`: keyword to skip a block of code to continue the iteration
            * `break` expression: next to `break` keyword, the expression can be evaluated to a result
            * ex.
            ```
            fn main() {
                let mut counter = 0;

                let result = loop {
                    counter += 1;

                    if counter == 10 {
                        break counter * 2;
                    }
                };

                println!("The result is {result}");
            }
            ```
            * loop label: 
                * for loops within loops, break and continue apply to the innermost loop at that point
                * begin with a single quote
                * can break outerloop by doing `break 'outerloop_label;`
                * ex. 
                ```
                fn main() {
                    let mut count = 0;
                    'counting_up: loop {
                        println!("count = {count}");
                        let mut remaining = 10;

                        loop {
                            println!("remaining = {remaining}");
                            if remaining == 9 {
                                break;
                            }
                            if count == 2 {
                                break 'counting_up;
                            }
                            remaining -= 1;
                        }

                        count += 1;
                    }
                    println!("End count = {count}");
                }
                ```  
        * `while`: replace the pattern of using a combination of `loop`, `if`, `else`, and `break`
            * ex.
            ```
            fn main() {
                let mut number = 3;

                while number != 0 {
                    println!("{number}!");

                    number -= 1;
                }

                println!("LIFTOFF!!!");
            }
            ```
        * `for`: loop over the elements of a collection
            * technically could use a `while` loop but it's error prone if the test condition or index value is incorrect
            * also slow because of an extra condition check for whether the index is within the bounds for every iteration
            * ex.
            ```
            fn main() {
                let a = [10, 20, 30, 40, 50];

                for element in a {
                    println!("the value is: {element}");
                }
            }
            ```
            * ex. Using the `Range` library
            ```
            fn main() {
                for number in (1..4).rev() {
                    println!("{number}!");
                }
                println!("LIFTOFF!!!");
            }
            ```
            * safe and concise

## Lesson 4
* Ownership: a set of rules that govern how a Rust program manages memory
    
    * Memory management:
        - garbage collection that regularly looks for no-longer-used memory as the program runs
        - the programmer must explicitly allocate and free the memory
        - **Rust**: 
        > system of ownerships with a set of rules that the compiler checks
    
    * How memory is stored:
        - Stack:
            * All data stored must have a known, fixed size
            * LIFO (last in, first out)
            * Think of "pile of plates"
        - Heap: 
            * Empty space big enough *allocated* for oncoming data
            * *Pointer* is the address to the allocated space
                * Pointers can be stored in a stack because of their known, fixed size
                * Follow the pointer to get to the actual data
            * Think of "reserving seats in a restaurant"
            * Accessing data in the heap is slower and more work because of bookkeeping
        - A processor can do a better job if the data accessed is closer to each other (stack > heap)
        - Ownership addresses:
            * Keeping track of what parts of code are using what data on the heap
            * Minimizing the amount of duplicate data on the heap
            * Cleaning up unused data on the heap so you don’t run out of space

    * Ownership rules:
        > * Each value in Rust has an owner. 
        > * There can only be one owner at a time. 
        > * When the owner goes out of scope, the value will be dropped.

    * Example: string literal VS `String`
        * string literal:
            * known, fixed size at compile time
            * immutable
        * `String`:
            * blob of memory
            * unknown size at compile time
            * might change during runtime
            * mutable
        * ex.
        ```
        {
            let s = String::from("hello"); // s is valid from this point forward

            // do stuff with s
        }                       // this scope is now over, 
                                // and s is no longer valid

        ```
        -> Rust calls a `drop` function automatically at the closing curly bracket, the memory requested for `s` is now returned
    
    * Move
        * If two variables share the same value:
            * ex.
            ```
            let s1 = String::from("hello");
            let s2 = s1;

            println!("{}, world!", s1);
            ```
            * pointer, the length, and the capacity that are on the stack are copied
            * data on the heap is NOT copied
            \
            \
            ![Two variables sharing the same data](https://doc.rust-lang.org/book/img/trpl04-04.svg)
            
            * not called "shallow copy" even though seems like it
            * "ownership is *moved* from s1 to s2"
    
    * Copy
        * is not automatic
        * automatic copying is often inexpensive (stack data only)
        * To "deep copy" a variable that uses heap data, not just the stack data, we need to use a `clone` method
            * ex.
            ```
            let s1 = String::from("hello");
            let s2 = s1.clone();

            println!("s1 = {}, s2 = {}", s1, s2);
            ```
            * visual indicator that some expensive code is being executed
            \
            \
            ![Deep copy a variable](https://doc.rust-lang.org/book/img/trpl04-03.svg) 
    
    * Move/Copy rules:
        > * Stack data can be copied by default
        > * Heap data can be moved by default
        > * If a type implemented the `Drop` trait, it cannot also implement the `Copy` trait
        > * Only simple scalar values and nothing requires allocation or is a form of resource can implement the `Copy` trait
            >   * ex.
            >     * All the integer types, such as `u32`.
            >     * The Boolean type, `bool`, with values true and false.
            >     * All the floating-point types, such as `f64`.
            >     * The character type, `char`.
            >     * Tuples, if they only contain types that also implement Copy. For example, (`i32`, `i32`) implements Copy, but (`i32`, `String`) does not.

    * Move/copy rules apply to passing a value to a function the same as assigning a value to a variable

    * Ownership transfer:
        * Value returned from a function can be transferred to another variable
        * But tedious if anything passed in has to be passed back to make use of it again
        * ex.
        ```
        fn main() {
            let s1 = gives_ownership();         // gives_ownership moves its return
                                                // value into s1

            let s2 = String::from("hello");     // s2 comes into scope

            let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                                // takes_and_gives_back, which also
                                                // moves its return value into s3
        } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
        // happens. s1 goes out of scope and is dropped.

        fn gives_ownership() -> String {             // gives_ownership will move its
                                                    // return value into the function
                                                    // that calls it

            let some_string = String::from("yours"); // some_string comes into scope

            some_string                              // some_string is returned and
                                                    // moves out to the calling
                                                    // function
        }

        // This function takes a String and returns one
        fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                            // scope

            a_string  // a_string is returned and moves out to the calling function
        }
        ```
    
    * Reference (`&`): use a value without transferring ownership
        * like a pointer, a reference is an address to where the data is stored
        * unlike a pointer, a reference is guruanteed to point to a valid value of a particular type for the life of that reference
        * the value it points to will not be dropped when the reference stops being used
        * "Borrowing": creating a reference
        * ex.
        ```
        fn main() {
            let s1 = String::from("hello");

            let (s2, len) = calculate_length(s1);

            println!("The length of '{}' is {}.", s2, len);
        }

        fn calculate_length(s: String) -> (String, usize) {
            let length = s.len(); // len() returns the length of a String

            (s, length)
        }


        --> Can be rewritten using reference like below:


        fn main() {
            let s1 = String::from("hello");

            let len = calculate_length(&s1);

            println!("The length of '{}' is {}.", s1, len);
        }

        fn calculate_length(s: &String) -> usize {
            s.len()
        } // Here, s goes out of scope. But because it does not have ownership of what
          // it refers to, it is not dropped.
        ```
        * immutable: trying to modify a reference will have an error `cannot borrow ``*some_string`` as mutable, as it is behind a `&` reference`
            * can make a reference mutable by doing so:
                * ex.
                ```
                fn main() {
                    let mut s = String::from("hello");

                    change(&mut s);
                }

                fn change(some_string: &mut String) {
                    some_string.push_str(", world");
                }
                ```
            * if you have a mutable reference to a value, you can have NO other references to that value in the same scope
                * ex. this works 🟢

                ```
                let mut s = String::from("hello");

                {
                    let r1 = &mut s;
                } // r1 goes out of scope here, so we can make a new reference with no problems.

                let r2 = &mut s;
                ```

                this does not work 🔴, will fail with an error `cannot borrow `s` as mutable more than once at a time`

                ```
                let mut s = String::from("hello");

                let r1 = &mut s;
                let r2 = &mut s;

                println!("{}, {}", r1, r2);
                ```
                * prevents "data race" where 3 behaviors can occur:
                    * Two or more pointers access the same data at the same time.
                    * At least one of the pointers is being used to write to the data.
                    * There’s no mechanism being used to synchronize access to the data.

                * same applies to COMBINING mutable and immutable references
                * ex. this works 🟢
                    ```
                    let mut s = String::from("hello");

                    let r1 = &s; // no problem
                    let r2 = &s; // no problem
                    println!("{} and {}", r1, r2);
                    // variables r1 and r2 will not be used after this point

                    let r3 = &mut s; // no problem
                    println!("{}", r3);
                    ```

                    this does not work 🔴, will throw an error `cannot borrow `s` as mutable because it is also borrowed as immutable`

                    ```
                    let mut s = String::from("hello");

                    let r1 = &s; // no problem
                    let r2 = &s; // no problem
                    let r3 = &mut s; // BIG PROBLEM

                    println!("{}, {}, and {}", r1, r2, r3);
                    ```

            * Dangling reference: a pointer that references a location in memory that may have been given to someone else
                * **NOT RUST**: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does
                
            * Slices: reference a contiguous sequence of elements in a collection rather than the whole collection
                * Range syntax (`..`): specify a starting index and/or an ending index (one more than the last position)
                    * ex. 
                    ```
                    let s = String::from("hello");

                    &s[0..2] and &s[..2] both give "he"

                    &s[3..len] and &s[3..] both give "lo"

                    &s[0..len] and &s[..] both give "hello"

                    NOTE: only works for ASCII, UTF-8 needs special handling
                    ```
                * literals and `String` values both can be sliced, so the code can be refactored to be more general and useful without losing any functionality
                    * ex.
                    ```
                    fn main() {
                        let my_string = String::from("hello world");

                        // `first_word` works on slices of `String`s, whether partial or whole
                        let word = first_word(&my_string[0..6]);
                        let word = first_word(&my_string[..]);
                        // `first_word` also works on references to `String`s, which are equivalent
                        // to whole slices of `String`s
                        let word = first_word(&my_string);

                        let my_string_literal = "hello world";

                        // `first_word` works on slices of string literals, whether partial or whole
                        let word = first_word(&my_string_literal[0..6]);
                        let word = first_word(&my_string_literal[..]);

                        // Because string literals *are* string slices already,
                        // this works too, without the slice syntax!
                        let word = first_word(my_string_literal);
                    }
                    ```
                * array slices
                    * ex.
                    ```
                    let a = [1, 2, 3, 4, 5];

                    let slice = &a[1..3];

                    assert_eq!(slice, &[2, 3]);
                    ```

    * Reference rules:
        > * At any given time, you can have either one mutable reference or any number of immutable references.
        > * References must always be valid.

## Lesson 5

* Struct: create custom data types meaningful for your domain by associating data and naming them clearly
    * like tuples, elements in a struct can be different types
    * unlike tuples, can name elements instead of position-based, relying on the order
    * "fields": the elements within a struct
    * ex.
    ```
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    ```
    * "instance": to use the struct, and the fields do not have to be defined in the same order as they were declared in the struct definition
    * ex.
    ```
    fn main() {
        let user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };
    }
    ```
    * use dot notation to access the element in a struct
        * the instance must be mutable to be able to change the element's value
    * ex.
    ```
    fn main() {
        let mut user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };

        user1.email = String::from("anotheremail@example.com");
    }
    ```
    * *field init shorthand* avoids having to repeat the values of the function parameters for the struct fields
    * ex.
    ```
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    --> does the same job as

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username: username,
            email: email,
            sign_in_count: 1,
        }
    }
    ```
    * *struct update* uses less code to copy a struct's values to a new struct instance
        * the `..user1` must come last
        * the fields of change can come in any order
        * the fields of change mean the ownership has moved to the new struct instance, so only the unchanged fields that have a `Copy` trait can be used in the source struct instance 
            * (ie, `user2` now has a new `email` value with a new `String` implementation
            * we can still use `user1`'s `active` and `sign_in_count` fields
            * but we cannot access `user1`'s `username` because `String` type does not have a `Copy` trait and its ownership has been transferred to `user2`
    * ex.
    ```
    fn main() {
        // --snip--

        let user2 = User {
            email: String::from("another@example.com"),
            ..user1
        };
    }

    --> does the same job as

    fn main() {
        // --snip--

        let user2 = User {
            active: user1.active,
            username: user1.username,
            email: String::from("another@example.com"),
            sign_in_count: user1.sign_in_count,
        };
    }
    ```
    * *tuple struct* is a tuple (no names associated to each field) with a struct name
        * good for differentiating tuples even if they're structurally the same (same data type, same number of elements)
        * avoid tedious naming of each element
        * ex.
        ```
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        fn main() {
            let black = Color(0, 0, 0);
            let origin = Point(0, 0, 0);
        }
        ```
    * *unit-like struct* behaves similarly to `()`, the unit type
        * useful for implementing a trait on some type that don't have any data that you want to store in the type itself
        * no need for curly brackets or parentheses
        * ex.
        ```
        struct AlwaysEqual;

        fn main() {
            let subject = AlwaysEqual;
        }
        ```
    * *Methods* are defined in the context of a struct (or an enum or a trait object)
        * `impl` block with a `self` in the parameter
        * `&self` is short for `self: &Self`
            * `&mut self`: enables changing the instance as part of what the method does
            * `self`: just self is rare, used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation
        * ex.
        ```
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }
        }

        fn main() {
            let rect1 = Rectangle {
                width: 30,
                height: 50,
            };

            println!(
                "The area of the rectangle is {} square pixels.",
                rect1.area()
            );
        }
        ```
        * the method name can be the same as the struct field name
            * "getters": name for this kind of method
                * useful when the field is private and method is public
            * **Rust**: not implemented automatically like other languages do
        * ex.
        ```
        impl Rectangle {
            fn width(&self) -> bool {
                self.width > 0
            }
        }

        fn main() {
            let rect1 = Rectangle {
                width: 30,
                height: 50,
            };

            if rect1.width() {
                println!("The rectangle has a nonzero width; it is {}", rect1.width);
            }
        }
        ```
        * defining multiple methods
            * the `&Rectangle` (referencing Rectangle) is used because we need `main` to keep the ownership of `rect2` so the `can_hold` method can use it
            * ex.
            ```
            impl Rectangle {
                fn area(&self) -> u32 {
                    self.width * self.height
                }

                fn can_hold(&self, other: &Rectangle) -> bool {
                    self.width > other.width && self.height > other.height
                }
            }

            fn main() {
                let rect1 = Rectangle {
                    width: 30,
                    height: 50,
                };
                let rect2 = Rectangle {
                    width: 10,
                    height: 40,
                };
                let rect3 = Rectangle {
                    width: 60,
                    height: 45,
                };

                println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
                println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
            }
            ```
        * *Associated function* is associated with the type named after the `impl`
            * `self` is not the first parameter (thus not a method)
            * NO need for an instance of the type to work with
            * ex. `String::from`
            * used for constructors that will return a new instance of the struct
                * `new`: is NOT a special name or built into the language
            * namespaced by `::`
            * ex.
            ```
            impl Rectangle {
                fn square(size: u32) -> Self {
                    Self {
                        width: size,
                        height: size,
                    }
                }
            }

            --> to call this associated function

            let sq = Rectangle::sq(3);
            ```
        
        * Multiple `impl` methods are allowed but mostly useful for generic types and traits 
        * ex.
        ```
        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }

            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
        }

        --> the same as

        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }
        }

        impl Rectangle {
            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
        }
        ```

## Lesson 6

* Enum: a value is one of a possible set of values
    * like structs, 
        * can group related data together, therefore custom data type
        * variants of the enum are namespaced under its identifier, and we use a (`::`) to separate the two
        * ex.
        ```
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        --> equivalent to but more concise than

        struct QuitMessage; // unit struct
        struct MoveMessage {
            x: i32,
            y: i32,
        }
        struct WriteMessage(String); // tuple struct
        struct ChangeColorMessage(i32, i32, i32); // tuple struct

        --> But if we used the different structs, each of which has its own type, we couldn’t as easily define a function to take any of these kinds of messages as we could with the Message enum, which is a single type
        ```
        * can implement methods through `impl`
        * ex. 
        ```
        impl Message {
            fn call(&self) {
                // method body would be defined here
            }
        }

        let m = Message::Write(String::from("hello"));
        m.call();
        ```
    * unlike structs,
        * can only be one of the variants
        * all variants are the same type
            * can be any type (string, numeric, struct, even another enum *etc.*)
        * ex. Can be implemented to take a value
        ```
        enum IpAddrKind {
            V4,
            V6,
        }

        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }

        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };

        --> can be re-written like

        enum IpAddr {
            V4(String),
            V6(String),
        }

        let home = IpAddr::V4(String::from("127.0.0.1"));

        let loopback = IpAddr::V6(String::from("::1"));
        ```

    * `Option` enum:
        * for a value that can be something or nothing
        * resolves a common bug in any programming language that implements `null`: 
            > assuming that something isn’t null when it actually is
        * so useful that it’s even included in the prelude
            * you don’t need to bring it into scope explicitly
        * `Option<T>`: in place of the `null`, it's an enum that encode a value can either be present or absent
            * ex.
            ```
            enum Option<T> {
                None,
                Some(T),
            }
            ```
                
            * `<T>` means that the `Some` variant of the `Option` enum can hold one piece of data of **any type**
            * that each concrete type that gets used in place of T makes the overall `Option<T>` type a different type
            * ex. Try adding a `i8` type to an `Option<i8>` type will throw an error
            ```
            let x: i8 = 5;
            let y: Option<i8> = Some(5);

            let sum = x + y;
            ```
        * Rust requires us to annotate the overall `Option` type
            * compiler can't infer what a `None` type should be for its corresponding `Some` variant
            * ex.
            ```
            let absent_number: Option<i32> = None;
            ```
        * Together with `match`, implement handling the case of the `Some<T>` variant and handling the case of the `None` variant
        * ex. 
        ```
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }

        let five = Some(5);
        let six = plus_one(five);  --> output Some(6)
        let none = plus_one(None);  --> output None


        --> Does Some(5) match Some(i)? It does! We have the same variant. The i binds to the value contained in Some, so i takes the value 5. The code in the match arm is then executed, so we add 1 to the value of i and create a new Some value with our total 6 inside.
        ```

    * `match`: compare a value with a series of patterns and execute code only when a (and the first) pattern is matched
        * unlike `if` statements, the conditions can be evaluated to ANY type other than only Boolean
        * anatomy
            * arm: pattern and code separated by `=>` operator
                * each code associated is an expression
                * if a code is one line, don't need `{}` and need a `(,)`
                * if a code is multiple lines, need `{}` and don't need a `(,)`
            * arms separated by `(,)`
        ```
        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => {
                    println!("Lucky penny!");
                    1
                }
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }
        ```
        * patterns that bind to values when they match
        * ex.
        ```
        #[derive(Debug)] // so we can inspect the state in a minute
        enum UsState {
            Alabama,
            Alaska,
            // --snip--
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }


        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                }
            }
        }

        --> if we call value_in_cents(Coin::Quarter(UsState::Alaska)), the "state" is now bound to "UsState::Alaska" because its pattern matched the value
        ```
        * matching must be exhaustive
        * ex. This will complain that we have not handled the `None` type
        ```
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                Some(i) => Some(i + 1),
            }
        }
        ``` 
        * catch-all and (`_`) placeholder: used when special conditions only apply to a few values and the rest take default actions
            * catch-all condition MUST be at the end, or else other conditions will not be evaluated
            * ex.
            ```
            let dice_roll = 9;
            match dice_roll {
                3 => add_fancy_hat(),
                7 => remove_fancy_hat(),
                other => move_player(other),
            }

            fn add_fancy_hat() {}
            fn remove_fancy_hat() {}
            fn move_player(num_spaces: u8) {}
            ```
            * (`_`) is a special pattern that matches any value and does not bind to that value
            * ex.
            ```
            let dice_roll = 9;
            match dice_roll {
                3 => add_fancy_hat(),
                7 => remove_fancy_hat(),
                _ => reroll(),
            }

            fn add_fancy_hat() {}
            fn remove_fancy_hat() {}
            fn reroll() {}
            ```
        * `if let`: allows less verbose `match` statements
            * however, it loses the exhaustive checking
            * syntatic sugar that checks for one pattern and ignores the rest
            * ex, do nothing for the rest
            ```
            let config_max = Some(3u8);
            match config_max {
                Some(max) => println!("The maximum is configured to be {}", max),
                _ => (),
            }

            --> can be rewritten as

            let config_max = Some(3u8);
            if let Some(max) = config_max {
                println!("The maximum is configured to be {}", max);
            }
            ```
            * ex, do the same thing for the rest
            ```
            let mut count = 0;
            match coin {
                Coin::Quarter(state) => println!("State quarter from {:?}!", state),
                _ => count += 1,
            }

            --> can be rewritten as

            let mut count = 0;
            if let Coin::Quarter(state) = coin {
                println!("State quarter from {:?}!", state);
            } else {
                count += 1;
            }
            ```
## Lesson 7

General idea:

```mermaid
graph TD;
    package-->binary crate;
    package-->library crate;
```

* **Crate**: smallest amount of code that the Rust compiler considers at a time
    * **Binary crate**: programs you can compile to an executable that you can run, such as a command-line program or a server. Must have a `main` function (what happens when the executables run).
    * **Library crate**: don’t have a main function, and they don’t compile to an executable. Define functionality intended to be shared with multiple projects 
    * **crate root**: a source file (implicit module name: `crate`) that the Rust compiler starts from and makes up the root module of your crate. The beginning of a "module tree".
        * binary: `src/main.rs`
        * library: `src/lib.rs`
    * each `.rs` file is a separate crate
* **Package**: a bundle of one or more crates that provides a set of functionality. Contains a `Cargo.toml` file that describes how to build those crates
    * can contain as many binary crates as you like, but at most only one library crate.
    * must contain at least one crate, whether that’s a library or binary crate

Rules of number and types of crates you can put per package:

```bash
├── my-project
│   ├── src
│   │   ├── main.rs (binary crate called "my-project")
│   │   ├── lib.rs  (library crate called "my-project")
│   │   ├── bin
│   │   │   ├── another_binary.rs
│   │   └── └── more_binary.rs
│   └── Cargo.toml
```
* **Module**: organize code within a crate for readability and easy reuse. Allows control the privacy of items.
    * Private items are internal implementation details not available for outside use.
    * Making modules and the items within them public exposes them to allow external code to use and depend on them
* **Path**: name the item within a module
* **`use`**: bring the item from a path into scope
* **`public`**: make an item public
* **Module Cheatsheet**:

    * **Start from the crate root**: When compiling a crate, the compiler first looks in the crate root file (usually `src/lib.rs` for a library crate or `src/main.rs` for a binary crate) for code to compile.
    * **Declaring modules**: In the crate root file, you can declare new modules; say, you declare a “garden” module with `mod garden;`. The compiler will look for the module’s code in these places:
        * Inline, within curly brackets that replace the semicolon following `mod garden`
        * In the file `src/garden.rs`
        * In the file `src/garden/mod.rs`
    * **Declaring submodules**: In any file other than the crate root, you can declare submodules. For example, you might declare mod vegetables; in `src/garden.rs`. The compiler will look for the submodule’s code within the directory named for the parent module in these places:
        * Inline, directly following `mod vegetables`, within curly brackets instead of the semicolon
        * In the file `src/garden/vegetables.rs`
        * In the file `src/garden/vegetables/mod.rs`
    * **Paths to code in modules**: Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an `Asparagus` type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`.
    * **Private vs public**: Code within a module is private from its parent modules by default. To make a module public, declare it with `pub mod` instead of `mod`. To make items within a public module public as well, use `pub` before their declarations.
    * The `use` keyword: Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use crate::garden::vegetables::Asparagus;` and from then on you only need to write `Asparagus` to make use of that type in the scope.

