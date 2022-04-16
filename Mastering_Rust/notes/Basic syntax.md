println!("{}",test);


when it encounters a format string like 

"{}", it calls a method from Display trait
"{:?}" it calls a method from Debug trait

![[Pasted image 20220402064531.png]]


match statement

catch all can be just '\_' or we can use a name

we could label a loop bock and just to that

we can destruct a tuple struct using let

let Color(a, b, c) = some color struct;

now a , b, c will hold the corresponding values


![[Pasted image 20220402082306.png]]

![[Pasted image 20220402082333.png]]

![[Pasted image 20220402104226.png]]



![[Pasted image 20220402115846.png]]


![[Pasted image 20220403064858.png]]

cargo test -- --nocapture to get prinlln ouputs during tests

![[Pasted image 20220403065923.png]]

![[Pasted image 20220403070255.png]]

should_painc, to check if the condition is panicing
ignorre, to ignore that test case

![[Pasted image 20220403070538.png]]

a single test function can alsoo be run by specifying the function name after cargo test


Traits

![[Pasted image 20220403094704.png]]



![[Pasted image 20220415055611.png]]


![[Pasted image 20220415055629.png]]


move

![[Pasted image 20220415075324.png]]


![[Pasted image 20220415075857.png]]

![[Pasted image 20220415081818.png]]

![[Pasted image 20220415083312.png]]

Dispatch

![[Pasted image 20220415085846.png]]

![[Pasted image 20220415092234.png]]

![[Pasted image 20220415092246.png]]


![[Pasted image 20220415092659.png]]

RAII

RAII stands for Resource Acquisition Is Initialization; a paradigm suggesting that resources must be acquired during initialization of objects and must be released when they are deallocated or their destructors are called.

![[Pasted image 20220415093726.png]]


Iterator invalidation

![[Pasted image 20220416011629.png]]


![[Pasted image 20220416015330.png]]


![[Pasted image 20220416015723.png]]

![[Pasted image 20220416020932.png]]


![[Pasted image 20220416021056.png]]

Borrowing

The macros print!, println!, eprint!, eprintln!, write!, writeln! and format! are a special case and implicitly take a reference to any arguments to be formatted.
let a = 10;
if we use printlln!("{}", a);

it will acutally take &a under the hood

![[Pasted image 20220416064839.png]]

![[Pasted image 20220416070152.png]]

match expression move all values by default so we can use ref keyword to explically amke it a reference


![[Pasted image 20220416072757.png]]

'static means it is alilive till the whole programm

![[Pasted image 20220416073008.png]]

![[Pasted image 20220416073118.png]]

![[Pasted image 20220416073405.png]]

life time subtyping

![[Pasted image 20220416081517.png]]

![[Pasted image 20220416083019.png]]

