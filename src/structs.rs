struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32{
        &self.width * &self.height
    }
    fn perimeter(&self) -> i32 {
        2 * (&self.height + &self.width)
    }
}

pub fn structs() {
    let rect = Rectangle {
        width: 60,
        height: 40,
    };
    println!("Area of given rectangle is {}", rect.area());
    println!("Perimeter of given rectangle is {}", rect.perimeter());
}
