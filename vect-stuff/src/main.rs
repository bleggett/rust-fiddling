fn main() {
    let mut vec: Vec<String> = vec![
        String::from("Apple"),
        String::from("Bannana"),
        String::from("Mango")];

    for i in &mut vec {
        i.push_str(" Pie");
        println!("{}", i);
    }

    println!("LAST PIE: {}", vec[2]);
}
