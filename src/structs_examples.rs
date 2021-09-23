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
    //Its how tuple and stucts are different, structs help manage type
    // print_point(    Color(10,5,true));
    print_point(Point(10,5,true));
    println!("user3 object printing: {:?} ", user3);
    println!("user3 object printing: {:#?} ", user3);
    let mut rectangle = Rectangle { width: 100, height: 200 };
    let rectangle2 = Rectangle { width: 20, height: 20 };
    println!("Area of rectangle2: {}", rectangle2.area());
    println!("Area of rectangle1: {}", rectangle.area());
    println!("Area of rectangle1 plus one: {}", rectangle.area_plus_one());

    //Enable below code to know below discussed material
    //using method with self as first parameter would prevent caller using object
    // further on which(self) it is called upon.
    // println!("Area of rectangle1 plus one: {}", rectangle.area_plus_one_test());
    // println!("Area of rectangle1 plus one: {}", rectangle.area_plus_one());
    // println!("Area of rectangle1: {}", rectangle.area());

    println!("Can rectangle1: {:#?}, hold rectangle2: {:#?}, decision: {}", rectangle,
             rectangle2, rectangle.can_hold(&rectangle2));
}

struct Color(u32, u32, bool);
struct Point(u32,u32,bool);

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32{
        println!("weight: {} height: {}",self.width,self.height);
        self.width*self.height
    }
    fn can_hold(&self, other: &Rectangle ) -> bool{
        self.width>=other.width && self.height>=other.height
    }
}
/// Override Rectangle implementation any time with newer impl, it works as overloaded implementation
/// ie. all implementations are valid and none is allowed to use the same method name ie.
/// overriding.
impl Rectangle {
    fn area_plus_one(&mut self) -> u32{
        self.height=100;
        println!("weight: {} height: {}",self.width,self.height);
        (self.width*self.height)+1
    }
    //Valid method which takes self itself instead reference to self or mutable reference of self
    fn area_plus_one_test(self) -> u32{
        println!("weight: {} height: {}",self.width,self.height);
        (self.width*self.height)+1
    }
}
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u32,
    is_active: bool,
}

fn print_point(point: Point){
println!("{}    {}   {}",point.0,point.1,point.2)
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
