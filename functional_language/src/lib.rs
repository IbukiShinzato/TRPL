use std::collections::HashMap;
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
fn simulated_expensive_caluculation(intensity: u32) -> u32 {
    println!("caluculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// fn generate_workout(intensity: u32, random_number: u32) {
//     // let expensive_result = simulated_expensive_caluculation(intensity);
//     // クロージャとは関数自体を変数として持たせること。
//     // let expensive_closure = |num| {
//     //     println!("caluculating slowly...");
//     //     thread::sleep(Duration::from_secs(2));
//     //     num
//     // };

//     // 型注釈を追加
//     let expensive_closure = |num: u32| -> u32 {
//         println!("caluculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     };

//     if intensity < 25 {
//         println!("Today, do {} pushups!", expensive_closure(intensity));

//         println!("Next, do {} situps!", expensive_closure(intensity));
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!("Today, run for {} minutes!", expensive_closure(intensity));
//         }
//     }
// }

pub struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // caluculationはクロージャ
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    // hashでargに対応する値を格納
    pub fn value(&mut self, arg: u32) -> u32 {
        let v = self.value.entry(arg).or_insert((self.calculation)(arg));
        *v
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    // Cacher::new()の引数は匿名関数、つまりクロージャ
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minitue",
                expensive_result.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    #[allow(unused_variables)]
    let v1 = c.value(1);
    let v2 = c.value(2);

    // v2はc.valueで2になっているはず

    assert_eq!(v2, 2);
}

#[test]
fn closure_test() {
    // クロージャで環境をキャプチャする
    let x = 4;
    // xがequal_to_xのスコープ内で使用可能 => キャプチャ
    // しかし、オーバーヘッドを起こす可能性がある
    let equal_to_x = |z| z == x;

    // // xはこのスコープ内では使用できない。
    // // 使用するならargsにxを入れる必要がある。 fn equal_to_x(x: i32, z: i32)
    // fn equal_to_x(z: i32) -> bool {
    //     z == x
    // }

    let y = 4;

    assert!(equal_to_x(y));
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
pub struct Shoe {
    size: u32,
    style: String,
}

pub fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter => vectorの所有権を奪う
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}

pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directory() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(sum, 18);
}
