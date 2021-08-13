All data stored on the stack must have a known, fixed size. data with an unknown size at compile time or a size that might change must be stored on the heap instead. 

the heap is less organized, when you put data on the heap, you request a certain amount of space. the memory allocatore finds an empty slop in the heap that is big enough, marks it as being in use and returns a pointer, which is the address of the location

this process is called allocating on the heap or allocating.

keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap and cleaning up unsed data on the heap so you dont run out of space are all problemns that ownership addresses. 

Ownership Rules

-> Each value in rust has a variable that's called its owner
-> There can only be one owner at a time
-> When the owner goes out of scope, the value will be dropped

`
{
	let s = String::from("testing"); // s is valid from this point
	// do somthing with s
} // this scope is over now, and s is no longer valid
`

when a variable goes out of scope, Rust calles a special function for us, this function is called drop

Rust calls drop automatically at the closing curly bracket

`
{
	let s = String::from("test");
	let a = s;
}
`

in this case as the string is allocated in the heap, only the pointer of the string is stored in the stack and as  variable a is assinged to s, it just creates another stack value with same pointer taken from s, so both a and s are pointing to same location in heap

this method is called shallow copy, were only the pointer are copied this increases performance

in deep copy the heap data is create again and new pointer is set, this costs performance for huge data sets

but because both variables are pointer to same location now, if drop is called twice, it will be double free memory corruption bug, 

Rust make it easy by considering variable s as no longer valid, thus it cant be accessed, thus Rust calls it as move instead of shallow copy

thus drop will be called only once.

to do deep copy we can use the clone method

this only applies to data types that doesnt support copy trait

like intergers , floats, they can be assigned to another variable and rust wont make the intial variable invalid... 

`
{
	let a = 10;
	let b = a;
	println!("{} {}",a ,b); // this works fine as interger values support copy trait
}
`

Function scops works same as the string variable, if pass a string to a function the owership is transfered and if some string is returned the ownership is brought

`
fn main() {
    let s1 = give_ownership(); // s1 is assinged to a string that is returned from a function
    let s2 = String::from("i create my ownership"); // create a new string s2, comes into scope

    let s3 = take_and_give_ownership(s2); // ownership of s2 is passed to a function and s3 will own the return value of the function
} //s1 goes out of scope, drop is called, s2 goes out of scope, its moved so no dro call, s3 goes out of scope, drop is called.

fn give_ownership() -> String {
    let temp = String::from("have this"); // variable comes into scope
    temp // ownership is returned 
} // temp goes out of scope, but its moved so no drop call

fn take_and_give_ownership(val: String) -> String {
    val // val comes into scope and the returned
} // no drop call
`

if we want to send a varibale to a function without transfering tis ownership we  can user references

the rules of references:
-> at given time, you have either one mutable reference or any number of immutable reference
-> references must always be valid (to prevent dangling pointers)

String slices also support transferring without owenship

str[string_index..EndingIndex];



