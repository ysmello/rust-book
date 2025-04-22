fn main() {
    let _guess: u32 = "42".parse().expect("Not a number!");

    // let another: u8 = "-1".parse().expect("Not a positive number!");
    println!("Number: {_guess}");

    let _f: bool = true;

    let _cat: char = '😻';
    println!("cat: {_guess}");

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
