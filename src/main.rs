use crate::packages_modules_crate::{ human as h};
use crate::packages_modules_crate:: human ::{self};
mod data_type_and_functions;
mod control_flow ;
mod slice;
mod structs_examples;
mod enum_examples;
mod packages_modules_crate;
mod vector_impl;

// use std::io;
// use std::collections::HashMap;
// use std::collections::*;
fn main() {
    // data_type_and_functions::entry_point();
    // control_flow::entry_point();
    // slice::entry_point();
    // structs_examples::entry_point();
    //Relative path
    // enum_examples::entry_point();
    //Absolute path
    // crate::enum_examples::entry_point();
    // crate::human::anshul::name();
    h::anshul::name();
    human::anshul::name();
}

