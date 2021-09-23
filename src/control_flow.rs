
#![allow(dead_code)]
pub(crate) fn entry_point() {
    condition_flow();
    let value = looping();
    println!("value must be 100000 {}",value);
    loop_with_for();
    loop_with_for_range();
    ownership();
    let s1=String::from("Anshul");
    ownership_trasnfered(s1);
    //ownership is transfered because of
    // println!("s1 is should not be accessible {}",s1);
    let x1 = 10;
    ownershipt_not_transfered(x1);
    // println!("value of x1 is accessable because of copy: {}",x1);


    race_condition_with_immutable_reference();
    race_condition_with_mutable_immutable_reference();
}

/// At any given time, you can have either one mutable reference or any number of immutable references.
/// References must always be valid.
fn race_condition_with_immutable_reference(){
    let mut s = String::from("Hello World");
    // race condition blocker can be overcome by encapsulating
    // mutation in different/non-parent scope
    {
        let r2 = &mut s;
        println!(" {}", r2);
    }
    let r1  = &mut s;
    println!(" {}", r1);
    // Two mutable references of same data in same scope are not allowed
    // let r2 = &mut s;
    // println!("{} {}", r1, r2);
}
fn race_condition_with_mutable_immutable_reference(){
    let mut s = String::from("Hello");
    let r1 = &s;
    let r2 = &s;
    println!(" {} {}", r1, r2);

    let r3 = &mut s;
    // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    // println!(" {} {} {}", r1, r2, r3);
    //Note that a reference’s scope starts from where it is introduced and
    // continues through the last time that reference is used.
    //So its okay to have mutable and immutable references as long as you are not using/accessing
    // immutable references after mutable defined.
    println!("{}", r3);

}
fn ownership_trasnfered(s: String){
    println!("value of string is owned {}",s)
}
fn ownershipt_not_transfered(x: i32){
    print!("{}", x);
}
fn ownership(){
    //this code works but not below
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{} world!!",s2);

    //Can't access variable after borrower ownership change
    // let s1 = String::from("hello");
    // let s2 = s1;
    //
    // println!("{}, world!", s1);



        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}", s1, s2);

}
fn condition_flow() {
    let n = 3;
    if n > 5 {
        println!("n is greater than 5 {}", n)
    } else {
        println!("n is smaller than 5 {}", n)
    }
    let dynamic = if true { 5 } else { 6 };
    println!("{}", dynamic)
}

fn looping() -> u128{
    let mut x: u128 = 1;
    loop {
        x = x + 1;
        println!("again {}", x);
        if x == 10 {
            //This statement works with or without semicolon
            break x
        }
    }
}

fn loop_with_for(){
    let a = [2,4,6,8,10,12];
    for element in a.iter(){
        println!("{}",element);
    }
}

fn loop_with_for_range(){
    for num  in (1..10).rev() {
        println!("Printing elements in reverse {}",num)
    }
}
