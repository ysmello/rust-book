fn main() {
    another_function();
    let total = sum(1, 2);
    println!("Total: {total}");
}

fn another_function() {
    println!("Another function.");
}

fn sum(a: i32, b: i32) -> i32 {
    let sum: i32 = { a + b };
    sum
}
