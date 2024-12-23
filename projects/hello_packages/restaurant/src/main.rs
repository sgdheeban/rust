
// Love it, renaming with as keyword
use restaurant::{eat_at_a_restaurant as eat_out, is_full};

fn main() {
    println!("Hello, world!");
    
    eat_out();

    println!("Calling from the outside client main.rs");

    is_full(true);
}