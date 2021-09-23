#compile with rustc
compile file: rustc <filename>.rs
run: .<filename>
### compile/build/run with cargo
```
cargo build
build project with dependencies and run main function.
cargo run
#Cargo also provides a command called cargo check. This command quickly checks your code to make sure it compiles but doesn’t produce an 
executable
cargo check 
#to release code
cargo build --release
```
Running cargo build for the first time also causes Cargo to create a new file at the top level: Cargo.lock
