// pub struct Counter {}

// // 関連型
// // 型の注釈をする必要がない
// impl Iterator for Counter {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item>;
// }

// // ジェネリクス型
// // 複数の型の実装が必要とされる
// pub trait Iterator<T> {
//     fn next(&mut self) -> Option<T>;
// }
