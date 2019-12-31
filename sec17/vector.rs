fn main() {
    let collected_iterator: Vec<i32> = (0..10).colloect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    println!("Vector Size: {:?}", xs.len());

    println!("Second element: {}", xs[1]);

    println!("Pop last element: {:?}", xs.pop());
}