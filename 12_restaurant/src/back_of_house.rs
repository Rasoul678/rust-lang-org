#[allow(dead_code)]
pub struct Breakfast {
    pub toast: String,
    #[allow(dead_code)]
    seasonal_fruit: String,
}

impl Breakfast {
    #[allow(dead_code)]
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

#[allow(dead_code)]
pub enum Appetizer {
    Soup,
    Salad,
}

#[allow(dead_code)]
fn fix_incorrect_order() {
    cook_order();
    super::deliver_order();
}
#[allow(dead_code)]
fn cook_order() {}
