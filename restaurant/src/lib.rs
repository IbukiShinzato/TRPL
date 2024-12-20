mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        sessional_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                sessional_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

mod front_of_house;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // // 慣例的に良くない
    // // どこでadd_to_waitlistが定義されたか不明瞭だから
    // add_to_waitlist();
    // add_to_waitlist();
    // add_to_waitlist();

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    // toastのみ変更可能
    meal.toast = String::from("Wheat");
    // meal.sessional_fruit = String::from("grape");
    println!("I'd like {} toast please", meal.toast);

    // // sessional_fruitがprivateなためこちらからのconstractはできない。
    // let mut meal = back_of_house::Breakfast {
    //     toast: String::from("rice"),
    //     sessional_fruit: String::from("banana"),
    // };

    // enum自身のみをpublicにすると使用可能
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// // 同じ名前であればasを使って名前のダブりを消す
// use std::fmt;
// use std::io as IoResult;

// fn function1() -> fmt::Result {}

// fn function2() -> IoResult<()> {}

// use std::io;
// use std::io::Write;

// io自身をselfとおく
use std::io::{self, Write};

// glob演算子で全ての公開要素を持ち込む
//
use std::collections::*;
