#[macro_use]
extern crate text_io;

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32} ,
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn print(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}", home);

    let some_message = Message::Move {x: 12, y: 1};
    some_message.print();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("{:?}", some_number);
    println!("{:?}", some_string);
    println!("{:?}", absent_number);
    
    let my_coin = Coin::Quarter;
    println!("Value of my coin in cents is: {}", value_in_cents(&my_coin));
    eprintln!("my_coin = {:#?}", my_coin);
    println!("{:?}", my_coin);

    let five = Some(5);
    let six = plus_one(five);
    let mut none = plus_one(None);
    eprintln!("five = {:#?}", five);
    eprintln!("six = {:#?}", six);
    eprintln!("none = {:#?}", none);
    println!("six = {}", none.unwrap_or_default());

    let some_value = 10u8;
    let x = match some_value {
        1 => 10,
        2 => 20,
        3 => 30,
        _ => some_value
    };
    if let Some(5) = five {
        println!("hurray, it's {}", five.unwrap());
    }

    // count non quarters:
    let coins = [Coin::Quarter, Coin::Quarter, Coin::Dime, Coin::Nickel];
    eprintln!("count_non_quarters = {:#?}", count_non_quarters(&coins));

}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn count_non_quarters(coins: &[Coin]) -> u32 {
    let mut count = 0;
    for coin in coins {
        match coin {
            Coin::Quarter => (),
            _ => count +=1,
        }
    }
    count
}