mod front_of_house;

pub use crate::front_of_house::hosting;

fn deliver_order() {}

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        super::deliver_order();
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

use crate::back_of_house::Breakfast;

pub fn eat_at_restaurant() {
    hosting::seat_at_table();

    // Order a breakfast in the summer with Rye toast
    let mut meal = Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
 
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
 }
 
