#![allow(dead_code)]

pub(crate) fn entry_point() {
enum_message_example();
}

fn enum_message_example(){
    let x = Message::Quit;
    println!("{:#?}", x);
    println!("{:#?}", Message::Write("hello".parse().unwrap()));
    let m = Message::Write(String::from("anshul"));
    m.call();
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){
        println!("From call method: {:#?}", self);
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
