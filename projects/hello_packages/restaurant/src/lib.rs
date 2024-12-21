mod back_of_house {
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

pub fn eat_at_a_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_wait_list(true);

    //relative path
    front_of_house::hosting::add_to_wait_list(false);

    // relative path for serving
    front_of_house::serving::fix_order();
}
