use std::i32;

#[derive(Debug)]
enum IPAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum UsState {
    Texas(String, i32),
    Washington(String, i32),
    // --- Snip ---
}

enum Coin {
    Penny,
    Nickel,
    Dime, 
    Quater(UsState),
}

#[derive(Debug)]
enum UsState2 {
    Texas,
    Washington,
    // --- Snip ---
}

enum Coin2 {
    Penny,
    Nickel,
    Dime, 
    Quater(UsState2),
}

#[derive(Debug)]
struct Rectangle {
    height1: u32,
    width1: u32,
}

impl Rectangle {

    fn square(size :u32) -> Self {
        Self {
            height1: size,
            width1: size,
        }
    }

    fn area2(&self) -> u32 {
        self.height1*self.width1
    }

    fn wide_rect(&self) -> bool {
        self.width1 > 100
    }

    fn width1(&self) -> u32 {
        self.width1
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height1 > other.height1 && self.width1 > other.width1
    }

}

fn route(ip:IPAddrKind) {
    println!("{:#?}",ip);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!!!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(UsState::Texas(city, population)) => {
            println!("Quarter got attached to the state - {:#?}, {:#?}", city, population);
            25
        },
        Coin::Quater(UsState::Washington(city, population)) => {
            println!("Quarter got attached to the state - {:#?}, {:#?}", city, population);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1) 
    }
}

fn put_on_fancy_hat() {
    println!("Put on fancy hat");
}

fn remove_fancy_hat() {
    println!("Remove fancy hat");
}

fn move_to_other_player(x: i32) {
    println!("Moving to other player - {x}");
}


fn re_roll() {
    println!("Re-rolling")
}

fn roll_dice(x: i32) {
    match x {
        3 => put_on_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_to_other_player(other)
    }
}

fn roll_dice_2(x: i32) {
    match x {
        3 => put_on_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => re_roll(),
    }
}

fn roll_dice_3(x: i32) {
    match x {
        3 => put_on_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

fn count_coins_2(coin2: Coin2, mut count: i32) -> i32{
    if let Coin2::Quater(state) = coin2 {
        println!("Coin is a Quarter from {state:?}");
    } else {
        count+=1;
        println!("increasing counter, {count}");
    }
    count
}

fn main() {
    let scale = 2;
    let rect = Rectangle {
        height1: dbg!(30*scale),
        width1: 50,
    };

    let rect2 = Rectangle {
        width1: 10,
        height1: 40,
    };
    let rect3 = Rectangle {
        width1: 60,
        height1: 45,
    };

    println!( "{}", 
        area(&rect)
    );

    println!("rect1 is {rect:?}");
    
    let sq = Rectangle::square(3);     
    println!("Printing a square height: {} and width: {}", sq.height1, sq.width1);

    dbg!(&rect);

    println!("Area2 of rectangle is {}", rect.area2());

    println!("Width1 of rectangle is {}", rect.width1);

    if rect.wide_rect() {
        println!("It's a wider rectangle of width {}", rect.width1);
    }
    else {
        println!("It's not a wider rectangle of width {}", rect.width1());
    }

    println!("Can rect1 hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect.can_hold(&rect3));


    route(IPAddrKind::V4);
    route(IPAddrKind::V6);

    println!("Sort coin {}", value_in_cents(Coin::Penny));
    println!("Sort coin {}", value_in_cents(Coin::Nickel));
    println!("Sort coin {}", value_in_cents(Coin::Dime));
    println!("Sort coin {}", value_in_cents(Coin::Quater(UsState::Washington(String::from("WA"), 98012))));
    println!("Sort coin {}", value_in_cents(Coin::Quater(UsState::Texas(String::from("TX"), 78540))));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Six {:#?}", six);
    println!("Seven {:#?}", plus_one(Some(6)));

    println!("Six {:#?}", none);
    println!("None 2 {:#?}", plus_one(None));

    roll_dice(3);
    roll_dice(7);
    roll_dice(9);
    roll_dice_2(9);
    roll_dice_3(9);

    let config_max = Some(10);
    if let Some(max_val) = config_max {
        println!("Config maxed at {:#?}", max_val)
    }

    let mut count=0;

    count = count_coins_2(Coin2::Penny, count);
    count = count_coins_2(Coin2::Dime, count);
    count = count_coins_2(Coin2::Nickel, count);
    count = count_coins_2(Coin2::Quater(UsState2::Texas), count);
    count = count_coins_2(Coin2::Quater(UsState2::Washington), count);

    println!("final count outside {count}");


}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width1*rectangle.height1
}