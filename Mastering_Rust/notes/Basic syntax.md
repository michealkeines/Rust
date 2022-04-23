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

Raw pointers

![[Pasted image 20220417065713.png]]

![[Pasted image 20220417071545.png]]



drop is called on the last object that is in the code from top to bottom

![[Pasted image 20220417072003.png]]

![[Pasted image 20220417072145.png]]

![[Pasted image 20220417073920.png]]

![[Pasted image 20220417074021.png]]


![[Pasted image 20220417074109.png]]

![[Pasted image 20220417081528.png]]

![[Pasted image 20220417084428.png]]

Intermutability

![[Pasted image 20220417085136.png]]

![[Pasted image 20220417085236.png]]

refcell borrow is checked in runtime this if there is a immutable and mutable borrow in same scope, the compiler will not say anytihng, but in runtime it will panic and exit

programmers are responsible for this checks

![[Pasted image 20220417085900.png]]

dyn keyword

fn test(arg: Box<dyn test_trait>)

unlike generic traits, the compiler doesnt know the contrete type that will be passed

this helps the compilter to know that it iwll dispatched dynaimcally in the runtime


Error Handling

![[Pasted image 20220418092654.png]]

![[Pasted image 20220419114750.png]]

![[Pasted image 20220423004719.png]]

converting between option and result

![[Pasted image 20220423004943.png]]

 A panic in one thread does not affect the other threads and is isolated, except in cases where they corrupt a mutex lock on some shared data; it is implemented as a macro by the same panic! mechanism


![[Pasted image 20220423023735.png]]



![[Pasted image 20220423080833.png]]


we can use 

let a = 10;

let b = &a;

or we can use ref keyword

let ref b = a;


![[Pasted image 20220423081614.png]]


we can use ref and mut ref to borrow valuess inside match statements



let destructure pattern can be used in function paramters too

if let <destructive pattern> = expression {}

function test(<destructive pattern>: expression )


we can use loop statement and return the value 

loop {
	break 12;
}

this will return 12 as the return value

