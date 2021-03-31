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

Expressions evaluate to a resulting value, eg: 5 + 6 evalutes to a value, this let x = 5 + 6; is a expression, 6 itself is expression as it evaluate to 6, calling function is a expression, block of scopes are expressions

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

		
