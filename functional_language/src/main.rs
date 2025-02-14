extern crate functional_language;

#[allow(unused_imports)]
use functional_language::*;
use std::io;

fn main() -> io::Result<()> {
    // let simulated_user_specified_value = 10;
    // let simulated_random_number = 7;

    // generate_workout(simulated_user_specified_value, simulated_random_number);

    // let example_closure = |x| x;

    // let s = example_closure(String::from("Ibuki"));
    // println!("{}", s);

    // // example_closureはStringと推論しているので整数値での使用不可。
    // let n = example_closure(21);
    // println!("{}", n);

    // let closure = |moji: String| {
    //     println!("{}", moji);
    // };

    // closure("hello".to_string());

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    println!("------");

    for val in &v1 {
        println!("Got: {}", val);
    }

    println!("------");

    for val in &v1 {
        println!("Got: {}", val);
    }

    Ok(())
}
