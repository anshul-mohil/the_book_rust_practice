// To see backtrace run program with
//RUST_BACKTRACE=1 or RUST_BACKTRACE=full
//RUST_BACKTRACE=1 cargo run
#![allow(dead_code)]

// use std::io::ErrorKind;
use std::fs::File;
use std::io::ErrorKind;

pub(crate) fn entry_point() {
// panic_stack();
recoverable_error_examples();
}
fn panic_stack(){
    panic!("crash and burn");
}
fn recoverable_error_examples(){
    let file_path = "/Users/ansh/codebase/rust/the_book_rust_practice/some.txt";
    let result = File::open(&file_path);

    // let f = match result {
    //     Ok(file) => file,
    //     Err(error) => error
    // };
    let f = match result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&file_path){
                Ok(file) => file,
                Err(e) => panic!("Problem opening the file: {:?}", e),
            },
            //IDK why this is not working
            // ErrorKind::PermissionDenied => {
            //     let s = "anshul";
            //     println!("{}",s);
            // },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }

    };

    println!("{:#?}", f.metadata());
}
