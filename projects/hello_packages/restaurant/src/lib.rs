mod back_of_house {

    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup, 
        Salad
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }

        // borrow instead of move, which is the default
        pub fn print_menu(&self) {
            println!("Today's menu is {} and {}",&self.toast, &self.seasonal_fruit);
        }
    }

    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {
        println!("Cooking order");
    }
}

fn deliver_order() {
    println!("Deliver order");
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list(relative_path: bool) {
            println!("add_to_wait_list via relative path: {}", relative_path);
        }

        pub fn is_full(val: bool) {
            if val {
                println!("Restaurant is full.");
            } else {
                println!("Restaurant is not full.");
            }
        }

        //fn seat_at_table() {}
    }

    pub mod serving {
        use crate::back_of_house;

        pub fn fix_order() {
            back_of_house::fix_incorrect_order();
        }

        //fn take_order() {}

        //fn serve_order() {}

        //fn take_payment() {}
    }
}

// Cool re-exporting of a specific function from within a private module
pub use crate::front_of_house::hosting::is_full;

pub fn eat_at_a_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_wait_list(true);

    //relative path
    front_of_house::hosting::add_to_wait_list(false);

    // relative path for serving
    front_of_house::serving::fix_order();

    // accessing structs in another module
    // Order a breakfast with french toast
    // Borrowing instead of moving, while at it, let's make it mutable as well
    let mut breakfast = back_of_house::Breakfast::summer("french");
    back_of_house::Breakfast::print_menu(&breakfast);

    // our breakfast is still alive after borrow, unlike move 🦀
    println!("alive {:#?}", breakfast);

    // now let's try to mutate & print struct member 🦀
    breakfast.toast=String::from("english");
    println!("alive {:#?}", breakfast.toast);

    println!("Available appetizers are {:#?} and {:#?}", back_of_house::Appetizer::Soup, back_of_house::Appetizer::Salad);

    // String is a owenable type default to utf-8, growable and managed on heap by Rust, crazy, this is why the crab works!
    let crab_emoji = String::from("🦀");

    println!("Calling from inside eat_at_a_restaurant, inside view with some special unfilled seats for VIPs {}", crab_emoji);
    is_full(false);

}
