#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_function() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    #[should_panic]
    fn test_pc() {
        let small = Rectangle {
            width: 1.2,
            height: 3.0
        };
        let big = Rectangle {
            width: 3.4,
            height: 4.5
        };
        assert_eq!(!small.can_hold(&small),true);
    }
}
