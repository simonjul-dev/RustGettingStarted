enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}   

fn main() {
    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    //The option enum
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    //The match Control Flow Operator
    let coin = Coin::Quarter(UsState::Alabama);
    let value = value_in_cents(coin);
    println!("The value of the coin is: {} cents", value);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("\n{:?}, {:?}", six, none);
    //print!("{six}, {none}"); won't compile

    //Concise Control Flow with if let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("\nThe maximum is configured to be {max}");
    }

}