to define a closure we start with a pair of vertical pipes '|'

let test = |val| {
	println!("{}",val);
	val
};

we can call it as like a normal function ie test(pram) with params in bracket

closure dont require you to annotate types of the parameters or the return values like functions do

type annotations are required on functions because they're explicit interface exposed to your users.

fn  add_one_v1   (x: u32) -> u32 { x + 1 } // valid function
let add_one_v2 = |x: u32| -> u32 { x + 1 }; // valid closure
let add_one_v3 = |x|             { x + 1 }; // valid closure
let add_one_v4 = |x|               x + 1  ; // valid closure

closure can be minimal depending on the implementation

if the type is not explicitly mentioned, then it takes the first call type as it actually try, if we call the closure wiht different type of pram, we ill get an error

test(1); // int type will be infered and set this closure
test("string") // now if we call it with string, it will throw an error as it is already set to int

fn test() {
	 let x = 1;
	 let a = |z| z == x;
}

this way a closure can access the variable the are in the scope with having to pass it to it.

the variabls that are used inthe closure scope can either take the owership or just as immutable ref, to force the owership we use move

let x = 1;
let b = |z| z == x; //this will be immutable ref, here closure doesnt take owership

let b = move |z| z == x; // x will be moved and thus no longer valid outside of closure.


