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


![[Pasted image 20220403065923.png]]

![[Pasted image 20220403070255.png]]

should_painc, to check if the condition is panicing
ignorre, to ignore that test case

![[Pasted image 20220403070538.png]]

a single test function can alsoo be run by specifying the function name after cargo test


Traits

![[Pasted image 20220403094704.png]]

