fn create_box() {
    let _box = Box::new(3i32);

    // box1は破棄されており、メモリ解放済み
}

fn main() {
    let _box2 = Box::new(5i32);

    {
        let _box3 = Box::new(4i32);

        // _box3はメモリ解放済み
    }

    // _box2のメモリが解放される
}