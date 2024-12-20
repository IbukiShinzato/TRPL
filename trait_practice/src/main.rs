use std::fmt::Display;

use trait_practice::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("Ibuki"),
        content: String::from("I'm studying rust programing language!"),
        reply: true,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summerize());

    // let article = NewsArticle {
    //     headline: String::from("Penguins win the Stanley Cup Championship!"),
    //     location: String::from("Pittsburgh, PA, USA"),
    //     author: String::from("Iceburgh"),
    //     content: String::from(
    //         "The Pittsburgh Penguins once again are the best \
    //          hockey team in the NHL.",
    //     ),
    // };

    // println!("News article available! {}", article.summerize())
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summerize());
}

// // where句を使ったより明確なトレイト境界
// // 冗長
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// // where句を使用
// fn some_function<T, U> (t: &T, u: &U) -> i32 {
//     where T: Display + Clone,
//           U: Clone + Debug,
// }

// // トレイトを実装している型を返す
// // この場合はSummaryトレイトを実装している何かを返す
// fn returns_summerizable() -> impl Summary {
//     Tweet {
//         username: String::from("Ibuki"),
//         content: String::from("I'm studying rust programing language!"),
//         reply: true,
//         retweet: false,
//     }
// }

// // impl Trait構文の実装まわりの制約により許されていません。
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }
