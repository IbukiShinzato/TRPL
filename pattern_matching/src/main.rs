fn main() {
    // let favorite_color: Option<&str> = None;
    // let is_tuesday = false;
    // let age: Result<u8, _> = "34".parse();

    // // match式だと全てのパターンを網羅する
    // if let Some(color) = favorite_color {
    //     println!("Using your favorite color, {}, as the background", color);
    // } else if is_tuesday {
    //     println!("Tuesday is green day!");
    // } else if let Ok(age) = age {
    //     // ageは新しいスコープ内で有効
    //     if age > 30 {
    //         println!("Using purple as the background color");
    //     } else {
    //         println!("Using orange as the background color");
    //     }
    // } else {
    //     println!("Using blue as the background color");
    // }

    // let mut stack = Vec::new();

    // stack.push(1);
    // stack.push(2);
    // stack.push(3);

    // // popができなくなるまでつまりNoneが返ってくるまで
    // while let Some(top) = stack.pop() {
    //     println!("{}", top);
    // }

    // let v = vec!['a', 'b', 'c'];

    // // enumerate()で値とその値のindexをtupleで返す
    // // (index, value)
    // for (index, value) in v.iter().enumerate() {
    //     println!("{} is at index {}", value, index);
    // }

    // // letはマッチしたら束縛する命令
    // // これはOK
    // let (x, y, z) = (1, 2, 3);

    // // これは左辺と右辺がマッチしていないからエラー => mismatched types
    // let (x, y) = (1, 2, 3);

    // let some_option_value = None;
    // let Some(x) = some_option_value;

    // if let x = 5 {
    //     println!("{}", x);
    // }

    // // パターンを直接リテラルに合致させる
    // let x = 1;

    // match x {
    //     1 => println!("one"),
    //     2 => println!("two"),
    //     3 => println!("three"),
    //     _ => println!("anything"),
    // }

    // // 名前付き変数にマッチする
    // let x = Some(5);
    // let y = 10;

    // match x {
    //     Some(50) => println!("Got 50"),
    //     // ここでのyはy = 10のyではなく、あくまでyという変数があったらという仮定
    //     // もしここでx = Noneならここをスルーする
    //     Some(y) => println!("Matched, y = {:?}", y),
    //     _ => println!("Default case, x = {:?}", x),
    // }

    // println!("at the end: x = {:?}, y = {:?}", x, y);

    // // orを使用
    // let x = 2;

    // match x {
    //     1 | 2 => println!("one or two"),
    //     3 => println!("three"),
    //     _ => println!("anything"),
    // }

    // // ..　で範囲指定
    // let x = 3;

    // match x {
    //     // 1 | 2 | 3 | 4 | 5 と同等
    //     1..=5 => println!("one through five"),
    //     _ => println!("something else"),
    // }

    // struct Point {
    //     x: usize,
    //     y: usize,
    // }

    // let p = Point { x: 0, y: 7 };
    // // a = 0, b = 7
    // let Point { x: a, y: b } = p;
    // println!("{} = 0, {} = 7", a, b);

    // // x, yは変数名変わらず
    // let Point { x, y } = p;
    // println!("{} = 0, {} = 7", x, y);

    // // 構造体でのmatch式
    // struct Point {
    //     x: usize,
    //     y: usize,
    // }

    // let p = Point { x: 0, y: 7 };

    // match p {
    //     Point { x, y: 0 } => println!("On the x axis at {}", x),
    //     Point { x: 0, y } => println!("On the y axis at {}", y),
    //     Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    //     // _ => println!("On neither axis: ({}, {})", x, y)　だとxとyが使用不可なので上の方が良い
    // }

    // #[allow(dead_code)]
    // enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(i32, i32, i32),
    // }

    // let msg = Message::ChangeColor(0, 160, 255);

    // match msg {
    //     Message::Quit => println!("The Quit variant has no data to destructure."),
    //     Message::Move { x, y } => {
    //         println!("Move in the x direction {} and in the y direction {}", x, y);
    //     }
    //     Message::Write(text) => println!("Text message: {}", text),
    //     Message::ChangeColor(r, g, b) => {
    //         println!("Change the color to red {}, green {}, and blue {}", r, g, b)
    //     }
    // }

    // struct Point {
    //     x: i32,
    //     y: i32,
    // }

    // let points = vec![
    //     Point { x: 0, y: 0 },
    //     Point { x: 1, y: 5 },
    //     Point { x: 10, y: -3 },
    // ];

    // // iter()を使うと参照を走査する
    // let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
    // println!("{}", sum_of_squares);

    // // 使用しない引数はアンダースコア(_)
    // // 仮に使用しない引数を入れていてもコンパイラがWarningを出す
    // fn foo(_: i32, y: i32) {
    //     println!("This code only uses the y parameter: {}", y);
    // }

    // foo(3, 4);

    // // 値の一部だけを無視
    // let mut setting_value = Some(5);
    // let new_setting_value = Some(10);

    // match (setting_value, new_setting_value) {
    //     // どちらともNoneでない
    //     (Some(_), Some(_)) => {
    //         println!("Can't overwrite an existing customized value");
    //     }

    //     // どちらかがNone
    //     _ => {
    //         setting_value = new_setting_value;
    //     }
    // }

    // println!("setting is {:?}", setting_value);

    // // 複数箇所無視すること可能
    // let number = (2, 4, 8, 16, 32);

    // match number {
    //     (first, _, third, _, fifth) => println!("Some numbers: {}, {}, {}", first, third, fifth),
    // }

    // // アンダースコア(_)を変数名の前につけるとWarningが出なくなる。
    // // ただし値を変数に束縛している
    // // 後に使う変数などにつけておくと良い
    // let _x = 5;
    // let y = 10;

    // let s = Some(String::from("Hello!"));

    // if let Some(_s) = s {
    //     println!("found a string");
    // }

    // // _sにmoveされているので使用不可
    // println!("{:?}", s);

    // if let Some(_) = s {
    //     println!("found a string");
    // }

    // // moveされていないので使用可能
    // println!("{:?}", s);

    // struct Point {
    //     x: i32,
    //     y: i32,
    //     z: i32,
    // }

    // let origin = Point { x: 0, y: 0, z: 0 };

    // // y, zは無視したいので「..」を使用
    // // y: _, z: _ と置かないでも良い
    // match origin {
    //     Point { x, .. } => println!("x is {}", x),
    // }

    // let numbers = (2, 4, 8, 16, 32);

    // match numbers {
    //     // 4, 8, 16は無視
    //     (first, .., last) => println!("Some numbers: {}, {}", first, last),
    // }

    // match numbers {
    //     // どこからどこまでを無視すべきなのかはコンパイラが理解できない
    //     // なので、「..」は一回のみしか使えない。
    //     (.., second, ..) => println!("Some numbers: {}", second),
    // }

    // let mut robot_name = Some(String::from("Bors"));

    // match robot_name {
    //     // // matchの参照は　「&」　ではなく　「ref」　を使う
    //     // Some(ref name) => println!("Found a name: {}", name),

    //     // 可変参照を取得
    //     // 参照外し
    //     Some(ref mut name) => *name = String::from("Another name"),
    //     None => (),
    // }

    // // // nameにmoveされているので使用不可
    // // println!("robot_name is: {:?}", robot_name);

    // // ref を使用するとmoveされていないのでrobot_nameは使用可能
    // println!("robot_name is: {:?}", robot_name);

    // // マッチガード
    // let x = Some(5);
    // let y = 10;

    // match x {
    //     Some(50) => println!("Got 50"),
    //     // マッチガードを使用することによって外側の変数(y)と等しいかのチェックが可能
    //     Some(n) if n == y => println!("Matched, n = {:?}", n),
    //     _ => println!("Default case, x = {:?}", x),
    // }

    // println!("at the end: x = {:?}, y = {:?}", x, y);

    // let x = 4;
    // let y = false;

    // match x {
    //     // (4 | 5 | 6) if y　と同等
    //     4 | 5 | 6 if y => println!("yes"),
    //     _ => println!("no"),
    // }

    // @束縛
    enum Message {
        Hello { id: u32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            // @を使用すると変数を生成 and パターンに一致するかの確認
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            // @をつけていないのでidはどの数字になるかわからないので使用不可
            // println!("Found an id in another range {}", id)
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            // それ以外のidが見つかりました
            println!("Found some other id: {}", id)
        }
    }
}
