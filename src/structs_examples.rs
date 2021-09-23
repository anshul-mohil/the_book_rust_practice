#![allow(dead_code)]

pub(crate) fn entry_point() {
    // let user_name = "Anshul";
    // let email_address = "anshul.mohil@gmail.com";

    let user1 = User {
        username: String::from("Anshul Mohil"),
        email: String::from("anshul.mohil@gmail.com"),
        sign_in_count: 10,
        is_active: true,
    };

    println!("{}", user1.username);
    // user1.email = "anshul.mohil@outlook.com".parse().unwrap();
    println!("{}", user1.email);

    let user2 = User {
        username: String::from("Anukul Mohil"),
        email: String::from("anukul.mohil@gmail.com"),
        ..user1
    };

    let user3 = get_new_struct_based_on_other_struct(String::from("mann"), String::from
        ("mann@gmail.com"), user1);
    println!("{}{}{}", user3.username, user3.email, user3.sign_in_count);
    println!("{}{}{}", user2.username, user2.email, user2.sign_in_count);
    //Value of user1 is already moved. If you want to use user1 here you need to pass reference
    // to methods which involve using lifetime inside struct if coping values from references.
    // println!("{}{}{}", user1.username, user1.email, user1.sign_in_count);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u32,
    is_active: bool,
}

//TODO: will learn at chapter 10
// struct UserWithRef{
//     username: &str,
//     email: &str,
//     sign_in_count: u32,
//     is_active: bool
// }
///This function demostrate how same variable values automatically attached to struct
fn get_new_struct_based_on_parameter_values(username: String, email: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        is_active: true,
    }
}
///username and email is auto mapped via parameter/local scope variables
///rest is used from user1 object
fn get_new_struct_based_on_other_struct(username: String, email: String, user1: User) -> User {

    User {
        username,
        email,
        ..user1
    }
}
