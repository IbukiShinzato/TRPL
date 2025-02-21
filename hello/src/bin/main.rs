extern crate hello;
use hello::ThreadPool;

use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to listen");

        // println!("Connection established!");
        // handle_connection(stream);

        // // 新しいスレッドを生成して処理
        // // しかしこれだと無限にthreadを生成してしまう
        // thread::spawn(move || handle_connection(stream));

        pool.execute(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // 1024バイトのbuffer
    // 基本的なリクエストでは十分なサイズ
    let mut buffer = [0; 1024];

    // 内部の状態が可変するのを考慮しての「mut」
    stream.read(&mut buffer).expect("Failed to read");

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // start_lineとfilenameのみがリクエストにより変化するのでリファクタリング可能
    let (start_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "html/hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "html/404.html")
    };

    let mut file = File::open(filename).expect("Failed to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Failed to raad");

    let response = format!("{}{}", start_line, contents);

    stream.write(response.as_bytes()).expect("Failed to write");
    stream.flush().expect("Failed to flush");
}
