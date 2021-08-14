vectors are variable lenght data structure

Vec<T>
	
let a: Vec<i32> = Vec::new();
	
a.push(1); // push to the end of the list
	
a.get(0) - > returns a option<T>, thus if we try to access a unknown index, we will get none, instead of error
	
to store different type in vector, we can create a seperate enum and use that as the custom type in vectore
	
	