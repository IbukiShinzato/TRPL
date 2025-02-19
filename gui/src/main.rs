extern crate gui;
use gui::{Draw, Screen};

use std::io;

struct SelectBox {
    width: u32,
    height: u32,
    option: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

fn main() -> io::Result<()> {
    let screen = Screen {
        components: vec![Box::new(String::from("Hi"))],
    };

    screen.run();
    Ok(())
}
