
#[derive(Debug)]
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

#[derive(Debug)]
pub enum Appetizer {
    Soup,
    Salad,
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
        println!(
            "Today's menu is {} and {}",
            &self.toast, &self.seasonal_fruit
        );
    }
}

pub fn fix_incorrect_order() {
    cook_order();
    super::deliver_order();
}

fn cook_order() {
    println!("Cooking order");
}
