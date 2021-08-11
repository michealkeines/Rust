Rust is a statically typed language, which means that it must know the types of all variable at compile time.

if a variable is assigned to function that is gonna return int at runtime, we have to specify the type strictly if not it will error out;

let guess = "42".parse().expect("not a number"); // this is wll error as we havnt mentioned the type of the varible and the return vaule is dynamic and compiler will have no idea what iwll be returned

let guess: i32 = "42".parse().except("not a number"); // this will wokr file as the compiler knows that it should except as int value

Scalar types;

Integer:
	![[Pasted image 20210811081953.png]]
	
signed version -  will contain negative values
unsigned version - will only be positive values

float, f32, f64

bool, true, false

char , single quotes characters

compount types:

tuple and arrays

tupes are fixed length once delared, they cannot shrink or grow
it can hold different type 

let test: (i32, f32, u8) = (500, 3.5, 2);

they can be accessed with . dot notaiton

test.0;
test.1

arrays are also fixed type and they cant grow or shrink, but they all hold same data type

let arr: [i32; 5] = [1,2,3,4,5];

we specifies the i32 datatype and 5 length

array can be accessed using [] notation

arr[1]



