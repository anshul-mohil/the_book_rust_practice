
`````shell
# to create new project
$ cargo new <project_name>
# TO build project for debugging(with faster build time) 
$ cargo build
# Production release build
$ cargo build --release

# build project with dependencies and run main function.
$ cargo run

# Cargo also provides a command called cargo check. 
# This command quickly checks your code to make sure it compiles but doesn’t produce an executable
$ cargo check 
# with backtrace enabled.
$ RUST_BACKTRACE=1 cargo run
#to understand a specific cause of an error
$ rustc --explain E0308
#To increase verbosity of logs to see in more detail what has been failed.
$ cargo run --verbose
`````

##### Notes:
###### [Expression and statements](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#statements-and-expressions) 
The let y = 6 statement does not return a value, so there isn’t anything for x to bind to. 
This is different from what happens in other languages, such as C and Ruby, where the assignment returns the value of 
the assignment. In those languages, you can write x = y = 6 and have both x and y have the value 6; that is not the case in Rust.
Running cargo build for the first time also causes Cargo to create a new file at the top level: Cargo.lock
#####

[//]: # (<p>)

[//]: # (compile with rustc)

[//]: # (compile file: rustc <filename>.r)

[//]: # (run: .<filename>)

[//]: # (compile/build/run with cargo)


[//]: # (TODO)
[//]: # (Next up is Control flow)
