pub trait Summary {
    fn summerize_auther(&self) -> String;

    fn summerize(&self) -> String {
        // デフォルト実装
        format!("(Read more from {}...)", self.summerize_auther())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// // trait名も含める
// impl Summary for NewsArticle {
//     // それぞれの振る舞い
//     // fn summerize(&self) -> String {
//     //     format!("{}, by {} ({})", self.headline, self.author, self.location)
//     // }

//     fn summerize_auther(&self) -> String {}
// }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // それぞれの振る舞い
    // fn summerize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }

    fn summerize_auther(&self) -> String {
        format!("@{}", self.username)
    }
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// トレイト境界を使用して、メソッド実装を条件分けする

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}
