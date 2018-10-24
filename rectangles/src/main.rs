#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32
}

impl Rect {
    // fn area(&self) -> u32 {
    //     self.width * self.height
    // }

    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rect { width: 30, height: 50 };
    let rect2 = Rect { width: 10, height: 40 };
    let rect3 = Rect { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
