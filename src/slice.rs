#![allow(dead_code)]
pub(crate) fn entry_point() {
    index_on_mutable_reference();
    slice_over_mutable_reference();
}

fn slice_over_mutable_reference() {
    let mut s = String::from("hello world with slices");
    let r1 = get_first_word_slice(&s);
    println!("{}",r1);
    s.clear();
    //Below statement is error: You cannot have/use mutable immutable references of slice
    // if underlying datastructure is changed.
    // println!("{}",r1);
}

// fn get_first_word_slice(s: &String) -> &str{
fn get_first_word_slice(s: &str) -> &str{
    for (i, &item ) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// Showcase Problem when working without slices on mutable reference
fn index_on_mutable_reference() {
    let mut s = String::from("hello world");
    let r1 = get_first_word_index(&s);
    s.clear();
    println!("{}", r1);
}


fn get_first_word_index(s: &String) -> usize {
    let x = s.as_bytes();
    for (i, &item) in x.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
