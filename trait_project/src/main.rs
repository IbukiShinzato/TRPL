use std::ops::Add;

// #[derive(Debug, PartialEq)]
// struct Point {
//     x: i32,
//     y: i32,
// }

impl Add for Point {
    // Pointインスタンス用に+演算子をオーバーロードする
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    // fn add(self, rhs: RHS) -> Self::Output; rhs(right hand self)は右辺
    // mをmmに変換して足し算
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

#[allow(dead_code)]
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// スーパートレイトを使用して別のトレイト内で、あるトレイトの機能を必要とする
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

// Displayの実装
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// ニュータイプパターンを使用して外部の型に外部のトレイトを実装する
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    // assert_eq!(
    //     Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
    //     Point { x: 3, y: 3 }
    // );

    let person = Human;

    // Humanに実装したfly()を呼び出す
    person.fly();

    // 引数にはselfの不変参照が入るので&personを入れる
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // Animalトレイトのbaby_nameが欲しい
    // これだとDogのbaby_nameを返す。
    println!("A baby dog is called a {}", Dog::baby_name());

    // // Animal::baby_name()はメソッドではなく、関連関数
    // //　self引数がないのでどのAnimal::baby_nameが欲しいかコンパイラには推論できない
    // println!("A baby dog is called a {}", Animal::baby_name());

    // フルパス記法を使用してどのトレイトからの関数かを呼び出す
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    println!();

    let point = Point { x: 10, y: 3 };
    point.outline_print();

    println!();
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
