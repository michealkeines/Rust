borrow checker is used to chech the life of a varible that needs to used,
{---------------------------------|r
	let r;                //                          |
	{						//	------|x          |
		let x = 5;  		//		  |		      |
		r = &x      		//		   |           |
	}							//-----|            |
	println!("{}",r);		//                    |
}---------------------------------|

borrow checker checks if the variable used has a bigger lifetime

in this case x life time is shorter than life time of x, thus it wont compile

to solve this we will be explicitly tell the compiler by a sperate lifetime ketyword

let a: &i32; // without lifetime

let b: &'a i32: // with lifetime 'a

life time is denoted by quote ' followed by a char

life time anotation by iteselt doesnt do anything, 

for exampe lets say we have a function with paramter first that is a reference to a i32 with lifetime 'a and second paramter i32 reference that also has the lifetime 'a. the lifetime anotations indicate that the references first and second must both live as long as the generic lifetime that is 'a

We’ve told Rust that the lifetime of the reference returned by any function is the same as the smaller of the lifetimes of the references passed

`fn main() {
    let string1 = "abcd";
    let string2 = "xyz";

    let result = longest(string1,string2);
    println!("The longest string is : {}",result);
}

fn longest<'l>(s1: &'l str, s2: &'l str) -> &'l str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}`

here 'l life time makes the function to return a reference without knowing the exactly refernce to the string as its declare for the whole function, this is just for speaking with the compiler, this doesnt actuallly change anything.

thus the function doesnt need to know excatly how long x and y will live, only that some scope can be substituted for 'l that will satisfy this signature.

when returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime paramter for one for the parameters

for structs we can specify the life time simlarly

lifetime elision:

compiler uses three rules to infer some of the lifetime without us having to explicitly provide it, only when these are not met, the compiler will ask us to fix the lifetime

-> each paramter that is a reference get its own life time paramter
-> if there is excatly one input lifetime parameter, that life time is assigned to all output lifetimes
->if there is mutiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all ouput lifetime parameters.

static lifetime:
this reference can live for the entire duration of the program
all string literals have the static lifetime

let s: &'static str = "I have a static life time.";

the text of this string is stored directly in the program' binary which is laways available 
