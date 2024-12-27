pub mod hosting {
    use crate::front_of_house::back_of_house::counter::talk_to_seating_coordinator;

    pub fn add_to_wait_list() {
        talk_to_seating_coordinator();
        println!("Add to wait list in the front_of_house hosting service");
    }
}