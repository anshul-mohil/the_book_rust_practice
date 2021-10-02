// To see backtrace run program with
//RUST_BACKTRACE=1 or RUST_BACKTRACE=full
//RUST_BACKTRACE=1 cargo run
#![allow(dead_code)]

// use std::io::ErrorKind;
use std::fs::File;
use std::io::{ErrorKind, Error, Read};

pub(crate) fn entry_point() {
// panic_stack();
// recoverable_error_examples();
//     unwrap_and_expect();
    let result = read_file_content_and_return();
    println!("{:#?}",result);

    let result2 = read_file_content_and_return_with_operator();
    println!("{:#?}",result2);
}
fn panic_stack(){
    panic!("crash and burn");
}
//the ? at the end of the File::open call will return the value inside an Ok to the variable f. If an error occurs, the ? operator will return early out of the whole function and give any Err value to the calling code. The same thing applies to the ? at the end of the read_to_string call.
fn read_file_content_and_return_with_operator() -> Result<String, Error> {
    let mut string = String::new();
    File::open("hello.txt")?.read_to_string(&mut string)?;
    Ok(string)
}
fn read_file_content_and_return() -> Result<String, Error>{
    let file = File::open("hello.txt");
    // let mut file = match file {
    //     Ok(file) => file ,
    //     Err(error) => return Err(error)
    //     // Err(error) => error
    // };
    let mut file = match file {
        Ok(file) => file ,
        //Maybe with return keyword it only try to send and
        //error if satisfied match,not the file object which
        // without return it automatically send. which interns
        //hold File and Error objects
        //Without return keyword values would go inside type variable
        //so all return values must be compatible. With return keyword
        //values doesn't go to variable but calling method.
        Err(error) => return Err(error)
    };
    let mut s = String::from(" ");
    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}
fn unwrap_and_expect(){
    //unwrap() if failed would panic!
    // let f = File::open("hello.txt").unwrap();

    //expect panic! by printing message
    let f = File::open("hello.txt").expect("Issue while opening file");
    println!("{:#?}",f.metadata())

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
            // ErrorKind::PermissionDenied => println!("{}","d"),
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }

    };

    println!("{:#?}", f.metadata());
}
