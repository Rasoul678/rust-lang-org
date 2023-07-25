use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    route(&home);
    route(&loopback);

    // Built-in
    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    route1(&localhost_v4);
    route1(&localhost_v6);

    let m = Message::Write(String::from("Hello"));
    m.call();

    // The Option Enum and Its Advantages Over Null Values
    let _some_number = Some(5);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;

    let _x: i8 = 5;
    let _y: Option<i8> = Some(5);

    // let sum = x + y; // Won't compile

    // The match Control Flow Construct
    let coin = Coin::Quarter(UsState::Alaska);
    value_in_cents(&coin);

    // Matching with Option<T>
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    // Catch-all Patterns and the _ Placeholder
    let dice_roll = 9;

    match dice_roll {
        3 => println!("Three"),
        7 => println!("Seven"),
        other => println!("{}", other),
    }

    match dice_roll {
        3 => println!("Three"),
        7 => println!("Seven"),
        _ => println!("Other Values"),
    }

    match dice_roll {
        3 => println!("Three"),
        7 => println!("Seven"),
        _ => (),
    }

    // Concise Control Flow with if let
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("non-quarter count: {}", count);
}

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_kind: &IpAddrKind) {
    println!("{:#?}", ip_kind);
}

fn route1(ip_kind: &IpAddr) {
    println!("{:#?}", ip_kind);
}

#[allow(dead_code)]
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("{:#?}", self);
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
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
