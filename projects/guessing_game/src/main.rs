use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {

    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let secret_number = rand::thread_rng().gen_range(1..=100);

        let mut guess  = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_msg) => {
                println!("msg: {_msg}");
                continue;
            }
        };

        let x = 5; 
        let y = 12;

        println!("x : {x} and y+2 : {}", y+2);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too Small!".red()),
            Ordering::Greater => println!("{}","Too Big!".red()),
            Ordering::Equal => {
                println!("{}","You Win!".green());
                break;
            }
        }

    }

}