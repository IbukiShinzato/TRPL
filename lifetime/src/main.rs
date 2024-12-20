use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExpect<'a> {
    part: &'a str,
}

impl<'a> ImportantExpect<'a> {
    fn level(&self) -> i32 {
        3
    }

    // &selfのライフタイムを受ける
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");

    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExpect {
        part: first_sentence,
    };

    println!("{:?}", i);

    // {
    //     let x = 5;
    //     let r = &x;
    //     println!("r: {}", r);
    // }

    // let string1 = String::from("abcd");
    // let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);

    // let string1 = String::from("long string is long");
    // let result = "";

    // {
    //     let string2 = "xyz";
    //     // resultのライフタイムの長さはstring2のライフタイムになる
    //     let result = longest(string1.as_str(), string2);
    // }

    // println!("The largest string is {}", result);
}

// // ライフタイムがわからない状態
// // 'aによりxとyが同じライフタイムであることをコンパイラに伝えた
// // ライフタイムが小さい方が優先
// // 仮に戻り値がxでライフタイムがyより長くても、xのライフタイムはyの長さになる
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// 返す値が一定ならライフタイムの変化はなし
// 引数のライフタイムに依存
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    x
}
