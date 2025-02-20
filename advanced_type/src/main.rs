fn main() {
    // 型エイリアス　=> 型を別の名前に置き換える
    // 先頭文字は大文字 typeキーワードを使用　　　type エイリアス名　 = 型名
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    assert_eq!(x, y);

    // let f: Box<Fn() + Send + 'static> = Box::new(|| println!("hi"));
    // fn takes_long_type(f: Box<Fn() + Send + 'static>) {
    //     // --snip--
    // }
    // fn returns_long_type() -> Box<Fn() + Send + 'static> {
    //     // --snip--
    // }

    // // 型が長いので型エイリアスを使用して冗長化を減らす
    // type Thunk = Box<Fn() + Send + 'static>;
    // let f: Thunk = Box::new(|| println!("hi"));
    // fn takes_long_type(f: Thunk) {
    //     // --snip--
    // }
    // fn returns_long_type() -> Thunk {
    //     // --snip--
    // }

    // // unwrap()の中身
    // impl Option<T> {
    //     pub fn unwrap(self) -> T {
    //         match self {
    //             Some(val) => val,
    //             None => panic!("called `Option::unwrap()` on a `None` value"),
    //         }
    //     }
    // }

    // // 動的サイズ決定型を保持する変数を生成することはできない
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";

    // &strは開始時点とスライスの長さを格納している
    #[allow(unused_variables)]
    let s1: &str = "Hello there!";
    #[allow(unused_variables)]
    let s2: &str = "How's it going?";
}
