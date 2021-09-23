// use std::io;
#![allow(dead_code)]
pub(crate) fn entry_point() {
    constant_variables_and_mutability();
    integer_literals();
    math_operations();
    boolean_example();
    char_example();
    tuple_examples();
    // panic_example();
    array_example();
    func_with_parameters(10);
    let value = expression_example(2, false);
    println!("returned value {}",value);
    let plus_one_val = plus_one(value);
    println!("{}",plus_one_val);
}

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
        println!("value of Y retruned by expression: {}", cal);
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

fn array_example() {
    //explicit type declaration
    let a: [u32; 5] = [1, 2, 3, 4, 5];
    let b=["anshul";5];
    let c = [-10,-20];
    println!("{}",b[0]);
    println!("{}",a[2]);
    println!("{}",c[0]);
}
fn func_with_parameters(x: i32){
    println!("X value is: {}", x)
}
fn tuple_examples() {
    //tuple with common values
    let tup_common = (1, 2, 3, 4, 5);
    println!("{}", tup_common.0);
    //tuple with misc values
    let tup_misc: (i32, f32, char, bool) = (100, 100.333, 'A', true);
    //tuple value accessing
    println!("{}", tup_misc.0);
    println!("{}", tup_misc.3);
    //tuple deconstruction
    let (x, y, z, l) = tup_misc;
    println!("This should be same as above {}", x);
    println!("This should be same as above  {}", l);
    println!("{}", y);
    println!("{}", z);
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

    //Note: you cannot shadow variable with constant
    const Z: i32 = 12;
    println!("Z const value: {}", Z);
}

fn boolean_example() {
    let t = true;
    println!("{}", t);
    let f: bool = false; // with explicit type annotation
    println!("{}", f);
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

fn integer_literals() {
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
