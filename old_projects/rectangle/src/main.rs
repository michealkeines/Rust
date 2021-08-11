struct Rectangle {
    width: i32,
    height: i32
}
impl Rectangle {
    fn create(width: i32, height: i32) -> Rectangle {
        println!("Constuctor called");
        Rectangle {
            width, height //short hand intialization
        }
    }

    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 25 };
    let rect2 = Rectangle::create(20,12);
    let rect3 = Rectangle::create(45,89);

    println!("Area of Rectangle: {} ", rect1.area());
    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));

}
