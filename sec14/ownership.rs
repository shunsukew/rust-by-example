fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c`は破棄されメモリは開放される。
}

fn main() {
    // スタック上
    let x = 5u32;

    // コピー
    let y = x;

    println!("x is {}, and y is {}", x, y);

    // a はヒープ上の整数へのポインタ
    let a = Box::new(5i32);
    println!("a contains: {}", a);

    // 所有権ムーブ
    let b = a;

    // これはエラー
    // println!("a contains: {}", a);

    destroy_box(b);

    // エラー
    //println!("b contains: {}", b);
}