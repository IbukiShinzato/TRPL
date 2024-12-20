// リファクタリング(構造化)
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 関連関数(not method)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );

    // // tuple型
    // let rect1 = (30, 50);

    // println!(
    //     "The area of the ractangle is {} square pixels.",
    //     area(rect1)
    // );

    // struct型
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // println!("rect1 is {:#?}", rect1);

    // println!("The area of the rectangle is {} pixels.", rect1.area());
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// // tuple型
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// struct型
#[allow(dead_code)]
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
