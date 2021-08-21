we can write test function that will be called when cargo test is exuted

add #[test] before the function that we want to run

mod test {
	#[test]
	fn test_function() {
		assert_eq!(add(5), 6);
	}
}

assert, assert_eq, assert_ne are used to call any function check if output matches with the given value

`assert_eq!(<function to call>, <output expected>)`

we can add ! to negate the output

`[shouw_panic]`

this can be used to check is the code panics



	