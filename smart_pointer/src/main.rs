// #[derive(Debug)]
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use List::{Cons, Nil};

#[allow(dead_code)]
#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    #[allow(dead_code)]
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

#[allow(dead_code)]
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

#[allow(dead_code)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // mainスコープを抜けたら実行される
    fn drop(&mut self) {
        // CustomSmartPointerをデータ`{}`とともにドロップするよ
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// #[allow(dead_code)]
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// #[allow(unused_imports)]
// use List::{Cons, Nil};

// #[allow(dead_code)]
// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            // 警告: 割り当ての75％以上を使用してしまいました
            self.messenger
                .send("Warning: You've used up over 75% of your quata!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            // 切迫した警告: 割り当ての90%以上を使用してしまいました
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use std::cell::RefCell;

    // struct MockMessenger {
    //     sent_messages: RefCell<Vec<String>>,
    // }

    // impl MockMessenger {
    //     fn new() -> Self {
    //         Self {
    //             // sent_messages: vec![],
    //             sent_messages: RefCell::new(vec![]),
    //         }
    //     }
    // }

    // impl Messenger for MockMessenger {
    //     fn send(&self, message: &str) {
    //         // 可変参照を得る
    //         self.sent_messages.borrow_mut().push(String::from(message));
    //     }
    // }

    // // 二つの可変参照を得る
    // // 実行時にエラーを起こす
    // impl Messenger for MockMessenger {
    //     fn send(&self, message: &str) {
    //         let mut one_borrow = self.sent_messages.borrow_mut();
    //         let mut two_borrow = self.sent_messages.borrow_mut();

    //         one_borrow.push(String::from(message));
    //         two_borrow.push(String::from(message))
    //     }
    // }

    // #[test]
    // fn it_sends_an_over_75_percent_warning_message() {
    //     let mock_messenger = MockMessenger::new();
    //     let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    //     limit_tracker.set_value(80);

    //     // 不変参照を得る
    //     assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    // }
}

// // Rc<T> * RefCell<T> => 複数の所有者を持ったまま内部の値を変更することができる。
// #[derive(Debug)]
// enum List {
//     #[allow(dead_code)]
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }

use std::io;
use std::rc::Rc;
use std::{cell::RefCell, ops::Deref};
use List::{Cons, Nil};

// Listの先を変更したい
#[derive(Debug)]
#[allow(dead_code)]
enum List {
    #[allow(dead_code)]
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    #[allow(dead_code)]
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

use std::rc::Weak;

#[derive(Debug)]
#[allow(dead_code)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() -> io::Result<()> {
    // branchが親、leafが子
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // leaf strong = 1, weak = 0
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        // branch strong = 1, weak = 1
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        // leaf strong = 2, weak = 0
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );

        // weakはスコープを抜けるとdropされる
    }
    // leaf parent = None
    println!("leaf parent = {:?}", leaf.parent.borrow_mut().upgrade());

    // leaf strong = 1, weak = 0
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    // let leaf = Rc::new(Node {
    //     value: 3,
    //     parent: RefCell::new(Weak::new()),
    //     children: RefCell::new(vec![]),
    // });

    // println!("lead parent = {:?}", leaf.parent.borrow().upgrade());

    // let branch = Rc::new(Node {
    //     value: 5,
    //     parent: RefCell::new(Weak::new()),
    //     children: RefCell::new(vec![Rc::clone(&leaf)]),
    // });

    // *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    // println!("a initial rc count = {}", Rc::strong_count(&a));
    // println!("a next item = {:?}", a.tail());

    // let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    // println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // println!("b inital rc count = {}", Rc::strong_count(&b));
    // println!("b next item = {:?}", b.tail());

    // // 循環参照
    // if let Some(link) = a.tail() {
    //     *link.borrow_mut() = Rc::clone(&b);
    // }

    // println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // 循環参照が起こる直後にプログラムを終了
    // println!("a next item = {:?}", a.tail());
    // a.tail();

    // let value = Rc::new(RefCell::new(5));

    // // 第一引数が持っている値、第二引数が次のListのアドレス
    // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    // let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // // println!("reference count of a is {}", Rc::strong_count(&a));

    // // value更新前
    // println!("a after = {:?}", a);
    // println!("b after = {:?}", b);
    // println!("c after = {:?}", c);

    // // RefCellによる可変参照の取得によりvalueを更新
    // *value.borrow_mut() += 10;
    // println!("\nchange value\n");

    // // value更新後
    // println!("a after = {:?}", a);
    // println!("b after = {:?}", b);
    // println!("c after = {:?}", c);

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // println!("{:?}", list);

    // let x = 5;
    // let y = MyBox::new(x);

    // xの値を指すボックス
    // let y = Box::new(x);

    // let x = 5;
    // let y = MyBox::new(x);

    // // *y == *(y.deref())
    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // let m = MyBox::new(String::from("Ibuki"));
    // hello(&m);

    // let m = "Ibuki";
    // hello(m);

    // let m = MyBox::new(String::from("Rust"));
    // println!("m = {:?}, *m = {}", m, *m);

    // hello(&(*m)[..]);

    // #[allow(unused_variables)]
    // let c = CustomSmartPointer {
    //     data: String::from("my stuff"),
    // };

    // #[allow(unused_variables)]
    // let d = CustomSmartPointer {
    //     data: String::from("othre stuff"),
    // };

    // mainスコープを抜けると、cとdに対してdrop()呼び出し
    // stackからのdropなのでdがcより先にdropされる。
    // println!("CustomSmartPointers created");

    // let c = CustomSmartPointer {
    //     data: String::from("some data"),
    // };
    // println!("CustomSmartPointer created.");
    // // 二重解放エラーになるので実行不可
    // c.drop();
    // // mainの終端の前にCustomSmartPointerがdropされた
    // println!("CustomSmartPointer dropped before the end of main.");

    // let c = CustomSmartPointer {
    //     data: String::from("some data"),
    // };
    // println!("CustomSmartPointer created.");
    // // std::mem::dropで強制drop
    // drop(c);
    // // CustomSmartPointerはmainが終わる前にドロップされた
    // println!("CustomSmartPointer dropped before the end of main.");

    // // cはdropされたので使用不可
    // println!("{:?}", c);

    // bにaの所有権が渡り、cは使用不可
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    // #[allow(unused_variables)]
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // // a生成後のカウント = {}
    // println!("count after creating a = {}", Rc::strong_count(&a));
    // #[allow(unused_variables)]
    // let b = Cons(3, Rc::clone(&a));
    // // b生成後のカウント = {}
    // println!("count after creating b = {}", Rc::strong_count(&a));
    // {
    //     #[allow(unused_variables)]
    //     let c = Cons(4, Rc::clone(&a));
    //     // c生成後のカウント = {}
    //     println!("count after creating c = {}", Rc::strong_count(&a));
    // }

    // // cがドロップされる。
    // // Dropトレイトの実装が自動で参照カウントを減らす。
    // // cがスコープを抜けた後のカウント = {}
    // println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // let x = 5;
    // // 不変値から可変参照を取得することは借用規則により不可能
    // let y = &mut x;
    // println!("{}", y);

    Ok(())
}
