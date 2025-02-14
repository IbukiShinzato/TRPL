// ライブラリクレートをバイナリクレートに持っていく
extern crate minigrep;

use std::env;
use std::io;
use std::process;

use minigrep::Config;

fn main() -> io::Result<()> {
    // // 希望の関数が2モジュール以上ネストされている場合、 関数ではなく親モジュールをスコープに導入するのが因習的です
    // let args: Vec<String> = env::args().collect();

    // 引数にargsのイテレータ
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // eprintln!にすることでエラーは画面に出力
        // 標準出力のみがファイルにリダイレクトされる。
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }

    Ok(())
}
