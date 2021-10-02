#![allow(dead_code)]
pub(crate) fn entry_point() {
    valid_lifetime_where_function_accessed_in_inner_or_smaller_scope();
    valid_lifetime_where_parameter_live_long_enough();
    lifetime_example_3();
}
fn lifetime_example_3() {
    let string1 = String::from("abcd-example-1 2");
    let result;
    {
        let string2= String::from("xyz");
        result= longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string outside scope {}", result);
}


fn valid_lifetime_where_function_accessed_in_inner_or_smaller_scope() {
    let string1 = String::from("abcd");
    {
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        let result2 = longest2(string1.to_string(), string2.to_string());
        println!("The longest string is {}", result);
        println!("The longest string is {}", result2);
    }
}

fn valid_lifetime_where_parameter_live_long_enough() {
    let string1 = String::from("abcd-example-1 2");
    let string2;
    let result;
    {
        string2= String::from("xyz");
        result= longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string outside scope {}", result);
}

//Using slices to avoid transferring ownership of passed parameter object
// fn longest(x: &str, y: &str) -> &str {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// transferring ownership of parameters
fn longest2(x: String, y: String) -> String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
