// front_of_house module containing other modules
// that contain functions
mod front_of_house;

mod back_of_house {

    // pub struct, members private by default
    pub struct Breakfast {
        pub toast: String,
        _seasonal_fruit: String,
    }
    // pub enum, members public by default
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            
            Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn _fix_incorrect_order() {
        
        _cook_order();

        // super is used as a relative path starting in the current module's parent 
        // in this case, parent is crate ("root").
        // like using ../ for a directory. 
        super::_deliver_order();  
    }

    fn _cook_order() {}
}

// with use we shorten the path to use
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {

    // Absolute path
    // the add_to_waitlist fn is in the same crate as eat_at_restaurant,
    // so we can use 'crate' to start an absolute path.
    crate::front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    
    // we can change toast as it's public
    meal.toast = String::from("Wheat");

    // we are not allowed to change seasonal_fruit (private)
    // meal.seasonal_fruit = String::from("blueberries");
    
    println!("I'd like {} toast please", meal.toast);

    let _order1 = back_of_house::Appetizer::Salad;
    let _order2 = back_of_house::Appetizer::Soup;
}

fn _deliver_order() {}
