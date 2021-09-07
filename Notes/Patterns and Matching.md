Pattern are used a lot in rust

match statment has a pattern => expression, it matches thorugh the given patterns

match test {
	Some(a) => 'if there is something',
	_ => 'all other cases'
}

if let all takes a pattern to run its arms

if let Some(a) = test {
	'is there is ome'
} else {
	'all other cases'
}

while let takes a pattern to run untill its not true

while let Some(a) = test {
	'till there is something you will see this message'
}

normal let declaration also matches pattern

let x = 5;
let (x,y) = (3,4); // matches a tuple pattern

Patterns come in two forms

refutable 
irrefutable

patters that wil match for any possible value passed are irrefutable

let x = 5;

here x would have matched any value as long as it is valid

patterns that will fail to match for some possivle value are refutable.

let test = 5;

if let Some(x) =test {} // this wont match as we are using a normal variable to match with some whihc also requires none to be defined

for loops only accept irrefutable patterns


match can use or or range in pattern

match test {
	1..5= => "one to five range",
	6 | 7 => "this or that or statement"
}

Destructing structs
	we can use struct definition as pattern to check its variables

Struct point {
	x: u32,
	y: u32
}

let p = point {x: 1, y: 3};

match p {
	point {x: 1, y:0} => 'this statment',
	point {x:1, y:3} => 'this one',
	point {x,y} => 'exact provided values'
}

enums can also be done this way, but the enum ordering should be maintained in the arms

Ignoring values in pattern:

_ underscore as a wild card pattern that will match match all rest of the values, also it doesnt bind to the values

it can be used in function parameters too

this is usefull for trait function that use paramters that might not be need for normal call thus we can use _ to make that paramter there to maintian the function signature, but also not use with out any errors

fn test (_:u32, val:u32) {}

we can use .. to ignore the rest of the value

(1,..)

match guard, if can used in match pattern to make it more compex condition
this can  also take variable out the scope to check a condition

let b = 5;

match test {
	some(a) if a > b => 'test',
	some(b) => 'test'
}

@ can be to both check for a condition and to bind the resutl of the contion to a variable

match test {
	test::what { val: temp @ 5..9 } => println!("{}", temp),
	test::what { val: 3 } => println!("{}",3)
}

here the temp value compare if the test has value betwwen 5 ot 9 and if it has it will bind it to temp var


