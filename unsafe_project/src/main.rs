#[allow(dead_code)]
unsafe fn dangerous() {}

// externでCの標準ライブラリから統合
extern "C" {
    #[allow(dead_code)]
    fn abs(input: i32) -> i32;
}

fn main() {
    // let mut num = 5;

    // // 参照から生ポインタを生成
    // // 有効であることが保証されている参照から生成された生ポインタ
    // // *constは不変参照、*mutは可変参照
    // let r1 = &num as *const i32;
    // let r2 = &mut num as *mut i32;

    // println!("r1 address is {:?}", r1);
    // println!("r2 address is {:?}", r2);

    // let address = 0x012345usize;
    // let r = address as *const i32;

    // println!("{:?}", r);

    // // 参照外しはunsafeブロック内でのみ可能
    // println!("r1 is {:?}", *r1);
    // println!("r2 is {:?}", *r2);

    // unsafe {
    //     println!("r1 is {:?}", *r1);
    //     println!("r2 is {:?}", *r2);
    // }

    // // unsafeな関数はunsafeブロック内で使用可能
    // dangerous();

    // unsafe { dangerous() }

    // let mut v = vec![1, 2, 3, 4, 5, 6];

    // let r = &mut v[..];

    // let (a, b) = r.split_at_mut(3);

    // // 任意のメモリアドレスからスライスを生成
    // use std::slice;

    // let address = 0x012345usize;
    // let r = address as *mut i32;

    // let slice = unsafe { slice::from_raw_parts_mut(r, 10000) };

    // println!("{:?}", slice);

    // // ffi(Foreign Function Language)はあるプログラミング関数を定義させ、異なる外部プログラミング言語にそれらの関数を呼び出す。
    // // ffiはunsafeブロック内で使用可能
    // unsafe {
    //     println!("Absolute value of -3 according to C: {}", abs(-3));
    // }

    // 可変で静的変数を読み込む
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
