use std::io;
// use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> io::Result<()> {
    // // 平行処理
    // // main threadが終わるまで実行
    // thread::spawn(move || {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // // handleで全ての結果を格納
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // // threadが完了するのを保証する
    // handle
    //     .join()
    //     .map_err(|_| io::Error::new(io::ErrorKind::Other, "Thread panic!"))?;

    // // main thread
    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // // move closureを使用
    // let v = vec![1, 2, 3];

    // // moveを使うことによってvの所有権の位置を把握させる
    // let handle = thread::spawn(|| {
    //     println!("Here's a vector {:?}", v);
    // });

    // handle
    //     .join()
    //     .map_err(|_| io::Error::new(io::ErrorKind::Other, "Thread panic!"))?;

    // let v = vec![1, 2, 3];
    // let handle = thread::spawn(move || {
    //     println!("Here's a vector {:?}", v);
    // });

    // // drop(v);

    // handle
    //     .join()
    //     .map_err(|_| io::Error::new(io::ErrorKind::Other, "Thread panic!"))?;

    // // 転送機と受信機
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     match tx.send(val) {
    //         Ok(_) => (),
    //         Err(_) => panic!("Failed to send"),
    //     };

    //     // 所有権が受信側(rx)にmoveされている
    //     println!("val is {}", val);
    // });

    // // main threadの実行をブロック
    // let received = match rx.recv() {
    //     Ok(message) => message,
    //     Err(_) => panic!("Failed to receive"),
    // };

    // println!("Got: {}", received);

    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];

    //     for val in vals {
    //         tx.send(val).err();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // // rxをイテレータとして扱っている
    // for received in rx {
    //     println!("Got: {}", received);
    // }

    // // 転送機(tx)をクローンして複数の生成器を作成
    // let (tx, rx) = mpsc::channel();

    // // 転送機(tx)生成
    // let tx1 = mpsc::Sender::clone(&tx);

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];

    //     for val in vals {
    //         match tx1.send(val) {
    //             Ok(_) => (),
    //             Err(_) => panic!("Failed to send"),
    //         };
    //         thread::sleep(Duration::from_secs(3));
    //     }
    // });

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("more"),
    //         String::from("messages"),
    //         String::from("for"),
    //         String::from("you"),
    //     ];

    //     for val in vals {
    //         match tx.send(val) {
    //             Ok(_) => (),
    //             Err(_) => panic!("Failed to send"),
    //         };
    //         thread::sleep(Duration::from_secs(2));
    //     }
    // });

    // // rxはイテレータとして扱っている
    // for received in rx {
    //     println!("Got: {}", received);
    // }

    // let m = Mutex::new(5);

    // {
    //     // lockを獲得
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    //     // Drop実装されているので、スコープを抜けると自動でアンロックされる
    // }

    // println!("m = {:?}", m);

    // let counter = Mutex::new(0);
    // let mut handles = vec![];

    // // for _ in 0..10 {
    // //     let handle = thread::spawn(move || {
    // //         let mut num = counter.lock().unwrap();

    // //         *num += 1;
    // //     });

    // //     handles.push(handle);
    // // }

    // let handle = thread::spawn(move || {
    //     let mut num = counter.lock().unwrap();
    //     *num += 1;
    // });
    // handles.push(handle);

    // // counterの所有権がhandleのクロージャにmoveされている
    // let handle2 = thread::spawn(move || {
    //     let mut num2 = counter.lock().unwrap();
    //     *num2 += 1;
    // });
    // handles.push(handle2);

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap());

    // // Rc<T>はスレッド間を共有するには安全ではない
    // let counter = Rc::new(Mutex::new(0));

    // Arc(Atomic Reference Counter)はスレッド間を安全に共有することができる
    let mut handles = vec![];
    let counter = Arc::new(Mutex::new(0));

    for _ in 0..10 {
        // let counter = Rc::clone(&counter);

        // ArcとRcは同じAPIなので基本的な関数は一緒
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // 内部で可変参照を取得
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    Ok(())
}
