fn eat_box(boxed_int: Box<i32>) {
    println!("Destroying box that contains {}", boxed_int);
}

fn borrow_box(borrowed_int: &i32) {
    println!("This int is: {}", borrowed_int);
}

fn main() {
    let boxed_int = Box::new(5);

    borrow_box(&boxed_int);
    borrow_box(&boxed_int);

    {
        let _ref_to_int: &i32 = &boxed_int;

        // エラー
        // eat_box(boxed_int);
    }

    eat_box(boxed_int);
}
