fn main() {
    let mut s1 = String::from("hello");
    let s2 = s1.clone();

    s1.push_str(", world");

    println!("s1: {s1}");
    println!("s2: {s2}");

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", &s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
