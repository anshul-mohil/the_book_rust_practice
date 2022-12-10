//Rust code uses snake case as the conventional style for function and variable names
// https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
// In function signatures, you must declare the type of each parameter.
// This is a deliberate decision in Rust’s design: requiring type annotations in function definitions
// means the compiler almost never needs you to use them elsewhere in the code to
// figure out what type you mean. The compiler is also able to give more helpful error messages
// if it knows what types the function expects.



pub(crate) fn entry_point() {
    func_with_parameters(10);
    println!("is 2 greater than five {}", is_greater_than_five(2));
    println!("is 10 greater than five: {}", is_greater_than_five(10));
}

fn func_with_parameters(x: i32) {
    println!("X value is: {}", x);
    // Calling a function is an expression.
    // Calling a macro is an expression.
    // A new scope block created with curly brackets is an expression
    // let x = (let y=6); //Wrong syntax: because assignment is a statement, doesn't return anything

    let y = {
        let x = x;
        //Note that the x + 1 line doesn't have a semicolon at the end, unlike most of the lines
        // you’ve seen so far. Expressions do not include ending semicolons. If you add a semicolon
        // to the end of an expression, you turn it into a statement, and it will then not return
        // a value. Keep this in mind as you explore function return values and expressions next
        x + 1
    };

    println!("The value of y={} from expression block adding 1 to value of x={}", y, x);
}

//https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#functions-with-return-values
fn is_greater_than_five(x: u32) -> bool {
    x > 5
}