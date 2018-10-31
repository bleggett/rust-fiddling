fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer {data: String::from("YOLOBAGGINS")};
    let d = CustomSmartPointer {data: String::from("NOLOBAGGINS")};
    println!("CustomSmartPointers created");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main");
}

struct MyBox<T>(T);

use std::ops::Deref;

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name)
}


struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}", self.data)
    }
}
