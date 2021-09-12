macros are there to make life easier

as rust macros write more rust code so that we dont have to

like vec![1,2,3];

Declarative macros

a vec macro can take any type and any number of input which is not possible to do, so we create a indirection, thus our macro will create new marco and push all the inputs we provided

to write marco we use macro_rules!

body is just arms like match expression, if arm matches it runs the respective code

the patters match against rust code structure and not values

macro_rules! name {
	//definition
}

we will see how vec was defined

macro_rules! vec {
	( $( $x: expr ), * ) => {
		{
			let mut temp = Vec::new(); // create a new vec
			$(temp.push($x);)* // * means it has zero or no of arguments provided
			temp
		}
	}
}

matching patter explanation 

( $( $x: expr ), * ) 
-> () everthing inside the first parentheses is matched so it wil match $ ($x:expr),*
->$ ( $x: expr ) this matched anything betwwen this parentheses sperately, here it will take any expr and will be accisible through $x
-> , this comma will match a literal comma as it sperates all the arguments passed to this macro
->* astrisk means there can be either zero or more number of arguments

thus vec![1,2,3,4]

this will takes 1 as expression as it sperated by , as $x and pass it to 

$(temp.push($x))

then 2 will be takes as expr as $x and pass ot 

$(temp.push($x))


macro_rules! test {
    ($($x:expr),+) => {
        {
        $(println!("{}",$x);)*
        }
    }
}

fn main() {
    test![1,2];

}


#[macro_export] it should be added to make the macro accessible wherever the crate is in scope


Procedural macros:

they accept some code as input operate on that code, and produce some code as an output

#[some_attribute]
pub fn somename(input: TokenStream) -> TokenStream {}

the source code that macro is operating on makes up the input tokenstream
and the code the macro produces is the output tokenstream

custom derive macro:

i we have a crate names hello_macro that defines a trait names HelloMacro with one associated function named test_me. Rather than making our crate users implement the hellomarco trait for each of their types, we will provide a procedural macro so users can annotate their type with 

#[derive(HelloMacro)] 

to get the default implemantion of the test_me function.

this can be used to achieve refection of dynamic class in name in compile time

we need bot ha normal hello_marcro trait and hello_macro_derive to be in same crate

Attribute like macro:

#[route(GET, "/")]

this can be used in functions too and has similar proc definition like custum derive

pub fn route(att: TokenStream, item: TokenStream) -> TokenStream {}

function like macro:
custom derive takes in rust code and matches pattern
function like macro takes in TokenStream and returns a tokenstream

let sql = sql!(Select * fomr posts where id=1);

this is sql query is parsed and checked for syntactic erros using complex process ing than normal macro_rule!

fn sql(input: tokenstream) -> TokenStream {}

