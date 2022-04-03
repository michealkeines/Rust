/// Implements a boolean `and` gate taking as input two bits and returns as ouput
pub fn and(a: u8, b: u8) -> u8 {
    a & b
}

/// Implements a boolean `xor` gate taking as input two bits and returning a it as ouput
pub fn xor(a: u8, b: u8) -> u8 {
    a ^ b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //#[should_panic]
    fn test_and() {
        assert_eq!(1, and(1,1));
        assert_eq!(0, and(0,1));
        assert_eq!(0, and(1,0));
        assert_eq!(0, and(0,0));
    }

    #[test]
    //#[should_panic]
    fn test_xor() {
        assert_eq!(1, xor(1, 0));
        assert_eq!(0, xor(0, 0));
        assert_eq!(0, xor(1, 1));
        assert_eq!(1, xor(0, 1));
    }
}
