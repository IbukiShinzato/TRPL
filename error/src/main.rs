use std::fs::File;
use std::io::{self, Read};

fn main() {
    // // 冗長性のあるコード
    // let f = File::open("hello.txt");

    // #[allow(unused_variables)]
    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
    //         Ok(fc) => fc,
    //         Err(e) => {
    //             panic!("Tried to create file but there was a problem: {:?}", e);
    //         }
    //     },
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error);
    //     }
    // };

    // // Result値がOkなら代入、Errならpanic!
    // // unwrapのpanicは全て同じmessage
    // let f = File::open("hello.txt").unwrap();

    // // expectでエラーの詳細を追加
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // // main関数に戻り値はないから利用できない。
    // let f = File::open("hello.txt")?;
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // // エラー委譲のショートカット: ?演算子
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // ?演算子は戻り値がResultのみ
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
