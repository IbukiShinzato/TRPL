// // 型によって関数を作るのは冗長
// #[allow(dead_code)]
// fn largest_i32(list: &[i32]) -> i32 {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if largest < item {
//             largest = item;
//         }
//     }

//     largest
// }

// #[allow(dead_code)]
// fn largest_char(list: &[char]) -> char {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if largest < item {
//             largest = item;
//         }
//     }

//     largest
// }

// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// // メソッド定義、Tをつけないといけない
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// enum Option_i32 {
//     Some(i32),
//     None,
// }

// enum Option_i64 {
//     Some(f64),
//     None,
// }

// // T, Uで複数の型のジェネリクス
// #[derive(Debug)]
// struct Point2<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point2<T, U> {
//     // 戻り値のフィールドに注意
//     fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
//         Point2 {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// ジェネリクス使用
// トレイト境界を追加
// PartialOrdとCopyトレイトを実装するあらゆるジェネリックな型に対して動く、 largest関数の実際の定義
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if largest < item {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let char_list = vec!['y', 'm', 'a', 'q'];
    // Traitを実装が必要
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest_i32(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest_char(&char_list);
    // println!("The largest char is {}", result);

    // // Traitを実装が必要
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    // let integer = Point { x: 5, y: 10 };

    // let float = Point { x: 1.0, y: 4.0 };

    // println!("{:#?}\n{:#?}", integer, float);

    // // xとyの方は共通でないといけない
    // let work_work = Point { x: 5, y: 4.0 };
    // println!("{:#?}", work_work);

    // let integer_and_float = Point2 { x: 5, y: 4.0 };
    // println!("{:#?}", integer_and_float);

    // let p = Point { x: 5, y: 10 };
    // println!("p.x = {}", p.x());

    // let p1 = Point2 { x: 5, y: 10.4 };
    // let p2 = Point2 { x: "hello", y: 'c' };

    // let p3 = p1.mixup(p2);

    // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // let integer = Option_i32::Some(5);
    // let float = Option_i64::Some(5.0);
}
