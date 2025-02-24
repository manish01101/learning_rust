mod front_of_house {
    pub mod hosting {
        pub fn add_to_waillist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn server_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waillist();
    // relative path
    front_of_house::hosting::add_to_waillist();
}

/* ---- here relative path will valid ---- */
// mod customer_experience {
//     mod front_of_house {
//         mod hosting {
//             fn add_to_waillist() {}
//             fn seat_at_table() {}
//         }
//         mod serving {
//             fn take_order() {}
//             fn server_order() {}
//             fn take_payment() {}
//         }
//     }
//     pub fn eat_at_restaurant() {
//         // absolute path
//         crate::front_of_house::hosting::add_to_waillist();
//         // relative path
//         front_of_house::hosting::add_to_waillist();
//     }
// }
// /* ---- here absolute path will valid ---- */
// mod dining {
//     pub fn eat_at_restaurant() {
//         // absolute path
//         crate::front_of_house::hosting::add_to_waillist();
//         // relative path
//         front_of_house::hosting::add_to_waillist();
//     }
// }

/* -- RELATIVE PATH WITH "SUPER" -- */
fn deliver_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // relative path with "super"
    }
    fn cook_order() {}
}

/* -- MAKING STRUCTS AND ENUMS PUBLIC -- */
mod back_of_house2 {
    pub struct Breakfast {
        /* A struct with some public fields and some private fields */
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
pub fn eat_at_restaurant2() {
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house2::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not
    // allowed to see or modify the seasonal fruit that comes
    // with the meal.
    // meal.seasonal_fruit = String::from("blueberries");
}
mod back_of_house3 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
pub fn eat_at_restaurant3() {
    let order1 = back_of_house3::Appetizer::Soup;
    let order2 = back_of_house3::Appetizer::Salad;
}

// “Bringing Paths into Scope with the use Keyword”
mod front_of_house4 {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house4::hosting;

pub fn eat_at_restaurant4() {
    hosting::add_to_waitlist();
}
// “use only creates the shortcut for the particular scope in which the use occurs
// “The compiler error shows that the shortcut no longer applies within the customer module:
// mod customer {
//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist();
//     }
// }

// there is no error as use used inside the customer
// “Notice there’s also a warning that the use is no longer used in its scope! To fix this problem, move the use within the customer module too, or reference the shortcut in the parent module with super::hosting within the child customer module.
// mod customer {
//     use crate::front_of_house4::hosting;
//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist();
//     }
// }
// or
mod customer {
    pub fn eat_at_restaurant() {
        super::hosting::add_to_waitlist();
    }
}

// CREATE IDIOMATIC USE PATHS
mod front_of_house5 {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
use crate::front_of_house5::hosting::add_to_waitlist;
pub fn eat_at_restaurant5() {
    add_to_waitlist();
}

// “Bringing two types with the same name into the same scope requires using their parent modules.”
// use std::fmt;
// use std::io;
// fn function1() -> fmt::Result {}
// fn function2() -> io::Result<()> {}

// PROVIDING NEW NAME WITH THE "as" KEYWORD
// use std::fmt::Result;
// use std::io::Result as IoResult;
// fn function1() -> Result {}
// fn function2() -> IoResult<()> {}
