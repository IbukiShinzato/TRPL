//使用機会が少ないので、手動でimport
use std::collections::HashMap;

fn main() {
    #[allow(unused_variables)]
    let v: Vec<usize> = Vec::new();
    // println!("{:p}:  {:?}", &v, v);

    #[allow(unused_variables)]
    let v = vec![1, 2, 3];
    // println!("{:p}:  {:?}", &v, v);

    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    {
        #[allow(unused_variables)]
        let v = vec![1, 2, 3, 4];

        // println!("{:p}:  {:?}", &v, v);
    }

    // println!("{:p}:  {:?}", &v, v);

    let v = vec![1, 2, 3, 4, 5];

    #[allow(unused_variables)]
    let third: &i32 = &v[2];
    // println!("The third element is {}", third);

    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element"),
    // }

    let mut v = vec![1, 2, 3, 4, 5];

    // println!("{:p}: {:?}", &v, v);

    v.push(6);

    // addressの位置は変わるはず?
    // println!("{:p}: {:?}", &v, v);

    // let v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{}", i);
    // }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // //enumを使って様々な型をvecに入れる
    // #[derive(Debug)]
    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }

    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Float(10.12),
    //     SpreadsheetCell::Text(String::from("blue")),
    // ];

    // println!("{:#?}", row);

    let s = String::new();

    #[allow(unused_variables)]
    let data = "initial contents";

    #[allow(unused_variables)]
    let s = s.to_string();

    #[allow(unused_variables)]
    let s = "initial contents".to_string();

    let mut s = String::from("foo");
    s.push_str("bar");

    s.push('l');
    // println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    #[allow(unused_variables)]
    let s3 = s1 + &s2;

    // println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    // println!("{}", s);

    #[allow(unused_variables)]
    let s = format!("{}-{}-{}", s1, s2, s3);
    // println!("{}{}{}", s1, s2, s3);

    #[allow(unused_variables)]
    let hello = "Здравствуйте";
    let hello = "hello";
    #[allow(unused_variables)]
    let s = &hello[0..1];
    // println!("{}", s);

    #[allow(unused_variables)]
    for b in "नमस्ते".bytes() {
        // println!("{}", b);
    }

    // 新規ハッシュマップを生成する
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 50);

    // println!("{:?}", scores);

    // vector -> HashMap
    let teams = vec![String::from("blue"), String::from("red")];
    let initial_scores = vec![10, 50];

    // コンパイラはvectorのデータ型に基づいて推測
    #[allow(unused_variables)]
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // println!("{:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    // println!("{:?}", map);

    // println!("{}, {}", field_name, field_value);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name);
    // println!("{:?}", score);

    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // println!("{:?}", scores);

    // // HashMapの上書き
    // scores.insert(String::from("Blue"), 50);
    // println!("{:?}", scores);

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 20);
    // println!("{:?}", scores);

    // scores.entry(String::from("Yellow")).or_insert(50);
    // println!("{:?}", scores);

    // scores.entry(String::from("Blue")).or_insert(50);
    // println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // mapのkeyになければvalueに0を挿入
        let count = map.entry(word).or_insert(0);

        // countにインクリメント (参照外し)
        *count += 1;
    }

    println!("{:?}", map);
}
