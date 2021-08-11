In rust variables are immutable by default.

let foo = 5; // immutable

let mut bar = 5; // mutable

mut token is used to make a varibale mutable

& - is used to send as reference 

you specify the signed and bit information after :

let guess: u32 = 13;

i32 - 32bit number
u32 - unsigned number
i64 - 64bit number

Constants are always immutable and type of the data that is being stored must be given;

const test: u32 = 19;

Shadoing is used to reassign a immutable varibale with another value or datatype, this is important because with way we have to use let again to assign the variable and if we dont we will get a compiler error

let a = 5; //imputable
a = a + 5; //error out

let a = a + 5; // shadowing

let a = "test"; //data type can allso be changed using shadowing


