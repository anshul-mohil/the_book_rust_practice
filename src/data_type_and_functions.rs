// use std::io;
#![allow(dead_code)]
// - [x] Complete task
// - [ ] Incomplete task When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to panic at runtime if this behavior occurs.
// Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping.
// To modify auto wrapping behaviour read further configurations https://doc.rust-lang.org/book/ch03-02-data-types.html
pub(crate) fn entry_point() {
    constant_variables_and_mutability();
    //  Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
    integer_float_examples();
    math_operations();
    boolean_example();
    char_example();
    // compound types
    tuple_examples();
    // panic_example();
    array_example();
    func_with_parameters(10);
    let value = expression_example(2, false);
    println!("returned value {}",value);
    let plus_one_val = plus_one(value);
    println!("{}",plus_one_val);
}
// Length	Signed	Unsigned
// 8-bit	i8	    u8
// 16-bit	i16	    u16
// 32-bit	i32	    u32
// 64-bit	i64	    u64
// 128-bit	i128    u128
// arch	    isize	usize
fn expression_example(x: i32, y: bool) -> i32{
    if y {
        return x;
    }
    else if x==10 {
        let cal = {
            let x = 3;
            // Expressions do not include ending semicolons
            x + 1
        };
        println!("value of Y returned by expression: {}", cal);
        return cal;
    }
    //last value as return value if without semicolon
    x+10
}
fn plus_one(x: i32)->i32{
    // return x+1;
//     OR
    x+1
}
// Arrays are useful when you want your data allocated on the stack rather than the heap
// or when you want to ensure you always have a fixed number of elements.
// An array isn’t as flexible as the vector type, though. A vector is a similar collection
// type provided by the standard library that is allowed to grow or shrink in size.
// If you’re unsure whether to use an array or a vector, chances are you should use a vector.
// https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type
fn array_example() {
    //Array definition syntax:
    // type of each element, a semicolon, and then the number of elements in the array
    //explicit type declaration
    let a: [u32; 5] = [1, 2, 3, 4, 5];
    println!("{}",a[2]);
    // implicit type identification by rust compiler
    //create array with same value 5 times
    let b=["anshul";5];
    let c = [-10,-20];
    println!("{}",b[0]);
    println!("{}",c[0]);
}
fn func_with_parameters(x: i32){
    println!("X value is: {}", x)
}
// You access tuple with 3 different ways:
// 1. use dot operator "." with index to access value in tuple
// 2 use variable name to access value inside tuple
// It seems tuples are to group variable, sort of non-growable variable list
// https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type
fn tuple_examples() {
    //tuple declaration with common values
    let tup_common = (1, 2, 3, 4, 5);
    println!("{}", tup_common.0);

    //tuple declaration with misc values
    let tup_misc: (i32, f32, char, bool) = (100, 100.333, 'A', true);
    //accessing tuple as DS
    println!("{}", tup_misc.0);
    println!("{}", tup_misc.3);

    //tuple is single entity, you can deconstruct tuple to extract each value and
    // map with variables
    //this is called tuple deconstruction
    let (x0,x1, x2,x3) = tup_misc;

    //accessing tuple internal var
    println!("Tuple as variable x0={}, tuple with position access tup_misc.0= {}", x0,tup_misc.0);
    println!("Tuple as variable x1={}, tuple with position access tup_misc.0= {}", x1,tup_misc.1);
    println!("Tuple as variable x2={}, tuple with position access tup_misc.0= {}", x2,tup_misc.2);
    println!("Tuple as variable x3={}, tuple with position access tup_misc.0= {}", x3,tup_misc.3);


    //Further study: Unit
    //The tuple without any values has a special name, unit. This value and its corresponding type
    // are both written () and represent an empty value or an empty return type.
    // Expressions implicitly return the unit value if they don’t return any other value.
}

fn constant_variables_and_mutability() {
    println!("Hello, world!");
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Shadowing
    let y = 15;
    println!("The value of shadow x: {}", y);
    let y = y * 2;
    println!("The value of shadow x: {}", y);

    /// can't shadow variable with constant
    /// must define datatype
    const Z: i32 = 12;
    println!("Z const value: {}", Z);
}

fn boolean_example() {
    let t = true;
    println!("{}", t);
    let f: bool = false; // with explicit type annotation
    println!("{}", f);
    let heart_eyed_cat = '😻';
    println!("{}",heart_eyed_cat)
}

fn math_operations() {
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;
    println!("{}", sum);
    println!("{}", difference);
    println!("{}", product);
    println!("{}", quotient);
    println!("{}", remainder);
}

fn integer_float_examples() {
    let guess: u32 = "10".parse().expect("not a number!");
    println!("Value of guess: {}", guess);
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Value of guess: {}", guess);
    let unsigned_x: i32 = 12772487;

    println!("value of ungined X: {}", unsigned_x);

    let test: i32 = 0b1111_0000;
    println!("binary defined test {}", test);
    let test: i64 = 0o77;
    println!("Octa  defined test {}", test);
    let test: isize = 0xff;
    println!("Hexa  defined test {}", test);
    let test: i8 = b'A' as i8;
    println!("Char  defined test {}", test);

    let x = 222_333_444.222_333_444_555;
    println!("x f64 format: {}", x);
    let y: f32 = 222_333_444.222_333_444_555;
    println!("y f32 format: {}", y);
    let y: f32 = 4.222_333_444;
    println!("y f32 format: {}", y);
}
/*
Rust’s char type is four bytes in size and represents a Unicode Scalar Value,
*/
fn char_example() {
    let c1: char = 'A';
    let c2 = 'ℤ';
    let c3 = '😻';

//     where as double quote is string
    let s1 = "Anshul";
    println!("{}", c1);
    println!("{}", c2);
    println!("{}", c3);
    println!("{}", s1);
}
// fn panic_example(){
//     let a = [1, 2, 3, 4, 5];
//
//     println!("Please enter an array index.");
//
//     let mut index = String::new();
//
//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");
//
//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");
//
//     let element = a[index];
//
//     println!(
//         "The value of the element at index {} is: {}",
//         index, element
//     );
// }
