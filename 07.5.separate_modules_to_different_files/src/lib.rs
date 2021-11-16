/*

Using a semicolon after mod front_of_house rather than using 
a block tells Rust to load the contents of the module from 
another file with the same name as the module.

*/
mod front_of_house;  

pub use crate::front_of_house::hosting;
use back_of_house::Breakfast;
use back_of_house::Appetizer;

pub fn eat_at_restaurant() {
    // Absolute path 
    //crate::front_of_house::hosting::add_to_waitlist();

    // Relative path 
    //front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // Order a breafast in the summer with Rye toast 
    // let mut meal = back_of_house::Breakfast::summer("Rye");
    let mut meal = Breakfast::summer("Rye");
    // Change our mind about whtat bread we'd like 
    meal.toast = String::from("Wheat");

    // The next line won't compile if we uncomment it;
    // we're not allowed to see or modify the seasonal fruit that
    // comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");

    // let order1 = back_of_house::Appetizer::Soup;
    // let order2 = back_of_house::Appetizer::Salad;
    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String, 
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast { // construct method
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}


