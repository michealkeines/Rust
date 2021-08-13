this is like a iterator we can give any value to and define a number of outcomes

match <contition> {
		cases
	}
	
match guess.cmp(1) {
		Ordering::Less => println!("value is less than one"),
		Ordering::Greater => println("value is greater then one"),
		Ordering::Equal => println("value is equal to one");
	}
	
it can any type of cases, if the condition will return a expetion
	
a exception could be of Result type which ahs two cases Ok and Err,
	which can be handles this way
	
	_ this can be used to handle all rest of the cases with a genreal output
	
	match coin {
		coin::nikel => 10,
		coin::dime => 2,
		_ => 0
	}
	
	
	
	
	
	
	