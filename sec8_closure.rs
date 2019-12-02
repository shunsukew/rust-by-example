use std::mem;

fn 

fn main() {
    fn function(i: i32) -> i32 {
        i + 1
    }

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;

    println!("function: {}", function(i));
    println!("annotated closure: {}", closure_annotated(i));
    println!("inferred closure: {}", closure_inferred(i));

    let one = || 1;
    println!("closure returning one: {}", one());

    let professor_x = "Charles Xavier";

    let print = || println!("Professor X's name is: {}", professor_x);

    print();


    // 8.2.1
    let color = "green";
    let print_color = || println!("`color`: {}", color);

    print_color();
    print_color();

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    inc();

    // 8.2.2
    fn apply<F>(f: F) where
        F: FnOnce() {

        f()
    }

    fn apply_to_3<F>(f: F) -> i32 where
        F: Fn(i32) -> i32 {

        f(3)
    }

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}.", greeting);

        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}