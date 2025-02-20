fn add_one(x: i32) -> i32 {
    x + 1
}

#[allow(dead_code)]
fn add_two(x: i32) -> i32 {
    x + 2
}

// 引数に関数を入れると良い
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// // クロージャを返そうとしているが、返却可能な具体的な型がないから実行不可
// fn returns_closure() -> Fn(i32) -> i32 {
//     |x| x + 1
// }

// クロージャを格納するサイズを知るためにBoxを使用
#[allow(dead_code)]
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    // add_one(5) + add_one(5) = 12
    let answer = do_twice(add_one, 5);

    let cl = |f: fn(i32) -> i32, arg: i32| f(arg) + f(arg);

    // (5 + 1) + (5 + 1) = 12
    println!("answer = {}", answer);

    println!("answer = {}", cl(add_one, 5));

    let list_of_number = vec![1, 2, 3];

    // クロージャでのstring変換
    let list_of_string1: Vec<String> = list_of_number.iter().map(|i| i.to_string()).collect();

    // フルパス記法でのstring変換
    // to_string()という利用可能な関数は複数あるから指定が必要
    let list_of_string2: Vec<String> = list_of_number.iter().map(ToString::to_string).collect();

    assert_eq!(list_of_string1, list_of_string2);
}
