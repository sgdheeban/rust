
mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list(relative_path: bool) {
            println!("add_to_wait_list via relative path: {}", relative_path);
        }

        //fn seat_at_table() {}
    }

    /*mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }*/
}

pub fn eat_at_a_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_wait_list(true);

    //relative path
    front_of_house::hosting::add_to_wait_list(false);
}
