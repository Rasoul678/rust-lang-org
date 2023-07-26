use crate::back_of_house::{Appetizer, Breakfast};

#[allow(dead_code)]
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    super::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
    #[allow(unused_variables)]
    let order1 = Appetizer::Soup;
    #[allow(unused_variables)]
    let order2 = Appetizer::Salad;
}
