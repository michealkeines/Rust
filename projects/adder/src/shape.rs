pub mod shape {
pub struct Rectangle {
    pub width: f32,
    pub height: f32
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}

}

