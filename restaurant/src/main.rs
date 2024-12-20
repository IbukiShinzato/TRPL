use std::collections::HashMap;

fn main() {
    // 構造体やenumはフルパス指定が慣例的
    let mut map = HashMap::new();
    map.insert(1, 2);

    println!("{:?}", map);
}
