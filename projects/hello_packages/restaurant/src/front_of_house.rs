
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
