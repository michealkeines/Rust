# Rust

let --> initialize a variable
const varname: type--> initialize a constant, should also mention the type to create a const 
mut --> make a variable mutable

Shadowing:
	let x = 5;
	let x = x + 1;
	
	in this case we are shadowing x by again using let to reassigin the x value, thus able to change the value of a immutable varible, and if we accidently make change to that variable without let, it will be a compile time erros, thus make its safe
	
	also while shadowing we change the intial type

	let spaces = "  ";
	let spaces = spaces.len();

	this reassiging with different type is not possible if the var is mutable

	let mut spaces = "  ";
	spaces = spaces.len();

Data type:
	while converting a value to another datatype you have to explicitly mention the type

	let guess = "42".parse().expect("not a number"); //this will error out as there is not type mentioned

	let guess: u32 = "42".parse().expect("not a number"); // this makes sure that varible is expecting a u32 interger

	char is stored as 'A' single quotes

	tuples once declared wont grow or shrink in size

	let test: (i32, f64, u8) = (500,6.5, 1);

	it can be accessed by dot operator

	test.0;

	array are fixed same type values grouped

	let arr = [1,2,3,3,4]; let test = [3; 5]; //this create a array with 5 3's

	it can be accessed using arr[0];

Statements are insturction that perform some action and do not return a value, eg: fn test() { let x = 5; } this whole funtion definition is a statement, this you cannot assign a let = x to another let like let x = (let y = 3); or let x = y = 3, which is possible in c


 a block like 
{
	let x = 3;
	x + 1
}
this block is a expression as it evaluate to something, notice there is not ';' at the end, if there is it would have been a statment

Functions:
	Function bodies are made up of series of statments optionally ending in a expression.

	//function declaration
	fn function_name( parameter ) { 
		// funtion body
	}

	function_name(parameters); // calling the function

	rust doesnt name the return value
	the return value of the function is synonymous with the value of the final expression in the block of the body of the function, you can return early from a function using the return keyword and specifing a value

	fn plus_one(x: i32) -> i32 { // return type is added after the arrows
		//body
		return value;
	}

	because rust is a expression based language, fn plus_one(x: i32) -> i32 { 5 } is a valid definition as the expression return 5 which meets the requirment which is i32, { x+4 } is also valid as it is still an expression, but { x+4; } is not valid as it will make that statment which doesnt evaluate to anything

Control statments:
	if condition {
		//true block
	} else {
		//false block
	}
	
	rust expects a boolean condition, numbers or string wont be converted to bool in rust

	if condition {

	} else if condition {

	} else {

	}
	because if is a expression it can used with let

	let test = if condition { // true } else { //false };

	if arms has to have same type, if not you will get a compiler error

	
	loop {
		//statements
	}
	this just repeats the statments indefinitly untill it get a break

	let mut c = 0; let result = loop { c += 1; if c == 10 { break c * 2; }  };

	loop can be used to assign value to variable, and break + expression can return that value

	while condition {
		//statements
	}

	for val in arr.iter() {
		//statements
	}

	for val in (1..5) {} for val in (1.5).rev()

Ownership:
	When you code calls a function, the values passed into the function and function's local variables get pushed onto the stack, when the function is over, the values get popped off the stack.
	keeping track of what parts of code are using what data on the heap, minimizing the amout of duplicate data on the heap, and cleaning up unused data on the heap so you dont run out of space are all problems that ownership addresses.
	Managing heap data is why ownership exists can help explain why it works that way it does.

Rule:
	Each value in rust has a variable that's called its owner
	there can only be one owner at a time
	when the owner goes out of scope, the value will be dropped

	let s = "test string"; // this allocation of memory cannot be mutated as it is stored in stack
	let s = String::from("test string"); // this allocation can be updated or deleted as it is stored in heap

	in order  to support mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents, thus means,
	-> the memory must be requested from the memory allocator at runtime
	-> we a way to returning the memory to the allocator when we're done with our string

	higher level programming languages use gc to  keep track of this memory and remove them automatically, but in c,c++, the developer has to deallocte the memory once its not used, which is a difficult problem as history sugguests(a lot of memory based security issues), if we forgot, we'll waste memory, if we do it early we'll have an invalid variable, if we do it twice, that's a bug too, we need to pair exactly one allocate with exacaty one free.

	Rust takes a diffrent path: the memory is automatically returned once the variable that owns it goes out of scope

	{
		let s = String::from("hello"); // s is valid

		//do stuff here 
	}		//s is no longer in scope, its freed
	
	rust calls a special function for us at the closing curly bracket, this function is called drop, a develper can put the code to return the memeory too
	
	when a heap alocated string is initialized, it has three parts, 1.pointer to the heap location 2.length 3.capacity
	these there informations are stored in stack
	

	
	there is two type of copy shallow and deep 

	in shallow copy only the information about the heap allocated memory is copied to the new variable

	let s1 = String::from("test");
	let s2 = s1; //shallow copy
	
	in deep copy the actually heap memory is create again and new pointer is stored in the stack

	let s1 = String::from("test");
	let s2 = s1.clone(); //deep copy but expensive

	so in rust if we are using shallow copy method for heap alloacte memory, the first instance of the memory will no longer be accessible, thus there is no need for deallocating it, only the s2 is freed with the scope is over

	let s1 = String::from("test");
	let s2 = s1;

	println!("{}",s1); //this will error as s1 is moved to s2 in rust anotation, thus wont be accessible

	because rust invalidates the first string, instead of being called as shallow copy, it's known as a move.

	this only applies for heap allocated type, not for types that can be stored in stack directly

	let a = 4;
	let b = a;
	println!("{} {}", a,b); // this is valid because interger can be copied without any great effort unlike heap

	Copy Trait:
		if a type implements copy trait, an older variable is still usable after assignment
		nothing that requires allocation or needs some form of resource can implement copy

	fn main() {
		let s = String::from("hello"); // s comes into scope

		takes_ownership(s); // s values moves into the function and so is no longer valid here

		let x = 5;  //x comes into scope
	
		makes_copy(x); // x would move into the function, x can be used again
	} // x goes out of scope, s not valid here, so no need for freeing
	
	fn takes_ownership(some_string: String){
		println!("{}",some_string)
	} // the some_string goes out of scope, drop is called and the memory is freed

	fn makes_copy(some_interger: i32){
		println!("{}", some_interger);
	} //here, some_integer goes out of scope, no freeing of memory 

	fn main() {
		let s1 = gives_ownership(); //takes owenership form for the returned value
		let s2 = String::from("hello"); 
		let s3 = takes_and_gives_back(s2); // s2 moves to function and the returned value is then placed in s3
	} // s1 goes out of scope, it is dropped, s2 moved to a function thus no drop needed, s3 get a value, this its dropped

	Ownership of a variable follows the same pattern every time: assiging a value to another variable moves it, when a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless the data has been moved to be owned by another variable.
	we can use references to void taking ownership, instead just take the value

	let s1 = String::from("hello");
	let len = calculate_len(&s); //by refercing s1 we just pass the pointer and not the ownership

	fn calculate_len(s: &string) -> usize {
		s.len()
	} // here s goes out of scope but the drop is not called  as s just take the refernce

	like variables, references are immutable by default, you can change that using mut keyword

	let mut s = String::form("test");
	let t = change(&mut s); // thus we are passing a mutable refrence so the function will be able to udpate the value of s

	but there is restriction in mutable references, you can have only one mutable reference within a scope

	let mut s = String::form("test");
	let r1 = &mut s;
	let r2 = &mut s;
	//this is not possible, only one mutable reference within a scope

	this prevents from race conditions

	let mut s = String::from("test");
	let r1 = &s;
	let r2 = &s;
	let mut r3 = &mut s; // this is not possible, you cant have a mutable and immutable refrence to a same variable if they are gonna be used below the mutable reference

	Dangling pointers:
		its easy to create a dangling pointer, a pointer that refrences a location in memory that may have been given to someone else.
	rust compiler gurantees that refrences will never be dangling refrence, complier will have proper compile time check based on lifetime

	string slice s[start..end];
	
Structures:
	it has different fields which then groups to become an object which can be used to access the values of each feild

	struct User {
		username: String,
		email: String,
		count: u64,
		active: bool
	}
	
	we can create an instance by key: value pairs

	let user1 = User {
		email: String::from("whatever@omg.todo");
		username: String::from("whatthelol");
		active: true,
		count: 1	
	};
	we dont have to define them in order

	we can also use dot notation to assign and access fields
	user1.email = String::from("pleasecomehere@place.loc");

	you can define and return struct as type

	fn test() -> User {} //this function return a User object 

	makeing same field names allows  you to add drictly use that instead of email: email we can use email
	
	struct test(i32, i32, i32); // tuple struct, these dont have field names but get the whole name i.e test in this case

	methods are functions inside struct context with self as the default parameter

	Associted functions are function defined inside a stuct but without self, thus they are used as contuctors that return a new instance of the structure
	they can accessed using :: thus String::new()
	


