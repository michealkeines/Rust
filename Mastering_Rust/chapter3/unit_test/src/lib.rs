// Let's test this function

fn sum(a: i8, b: i8) -> i8 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sum_input_objects() -> Vec<((i8,i8), i8)> {
        vec![
            ((1, 1), 2),
            ((0, 0), 0),
            ((2, -2), 0)
        ]
    }

    #[test]
    fn test_sums() {
        for (input, output) in sum_input_objects() {
            assert_eq!(sum(input.0, input.1), output);
        }
    }
}
