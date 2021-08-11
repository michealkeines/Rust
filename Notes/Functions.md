statments vs experssions

statments dont return anything 

let x = 5; // a statment

expressions evaluate to something 

let a = {
	let c = 2;
	c
};

this is a expression that is returning the value of c 

expressions dont have ; at the end

function can also return either with a explicit retun keyword or a experssion by the end of the function.


fn test() -> i32 {
	3
}

this is a valid function that returns i32 whihc will be 3


function parameters should specify the datatype 

fn test(x: i32) -> i32 {
	x + 1
}

this just take a x i32 and returns with with plus 1