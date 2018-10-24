fn main() {
    let s1 = String::from("hello");
    let s1 = take_ownership(&s1);
    println!("{}", s1);
}

fn take_ownership(some_string: &String) -> usize {
    println!("{}", some_string);
    some_string.len()
}
