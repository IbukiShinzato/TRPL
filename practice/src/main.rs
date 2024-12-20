fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // let some_u8_value = 1;
    // println!("{}", some_u8_value);
    // match some_u8_value {
    //     1 => println!("one"),
    //     3 => println!("three"),
    //     5 => println!("five"),
    //     7 => println!("seven"),
    //     _ => (),
    // }

    // 冗長性の多いコード
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // 冗長性の少ないコード
    if let Some(3) = some_u8_value {
        println!("three");
    }

    //以下の2つのコードは同等
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:#?}", state),
        _ => count += 1,
    }

    if let Coin::Quarter(state) = coin {
        println!("State quater from {:#?}", state);
    } else {
        count += 1;
    }
}
