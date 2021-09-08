to switch to unsage rust, use the unsafe keyword and then start a new block that holds the unsafe code.

there you can
-> dereference a raw pointer
-> call an unsafe method or function
-> access or modify mutable static variable
-> implement an unsafe trait
-> access fields of union

dereferencing raw pointers:

raw pointers can be declared using 
`*const ` as immutable pointer
`*mut` as mutable pointer

here the * dont mean it is dereferncing it, its just the declartion of raw pointers
`
let mut num = 5;

let r1 = &num as *const u32;
let r2 = &num as *mut u32; 
`

`let mut num = 5;


let add = 0x41414141;

let r = add as *const u32;
unsafe {
	println!("r2 : {}", *r);
    }`


unsafe keyword before functions

unsafe fn testing() {}

unsafe {
	testing();
}

fn split_at_mut(slice: &mut [i32], index: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(index <= len);

    (&mut slice[..index], &mut slice[index..])
}

this function takes a mutable list and splits into two halfs if the given index is less the nthe length of the list, but rust compiler wont be able to infer this, it only takes this return values as 

mutable reference to same list twice, which violates the borrow checking,

but this should be allowed and perfectly safe, when we know its completely safe and rust compiler doesnt understand it, we can use the unsafe code to make things happen as we want

fn split_at_mut(slice: &mut [i32], index: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(index <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, index),
            slice::from_raw_parts_mut(ptr.add(index), len-index)
        )
    }
}

this was we can write raw pointers unsafe code but with through valid reason that it wont fail

this was we can create unsage blocks inside safe functions that can be called to use in other places, this is how safe abstractions wrap around unsafe code

we can use extern keyword to call function from other languages

and all such functions will be considered unsafe bu default

extern "C" {
	fn abs(int: i32) -> i32;
}

we can make a rust function accessible in c program using same extern keyword

#[no_mangle]
pub extern "C" fn test_forC() {
	println!("tests");
}

we need to add no_mangle to make the compiler not change the name of the funciton as compilers change the fucntion names to have better context

static global variables can be mutable, thus making them unsafe and needs to in unsafe scope to manipulate them

static mut c: u32 = 0;

fn main() {
	unsafe {
		c += 1;
	}
	unsafe {
		println!("{}",c);
	}
}

a trait is unsage when atleast one of its method has some variant that the compiler cant verify. we can declare that trait as unsafe

unsafe trait test {
}

unsafe impl test for i32 {

}


unions are unsafe as the compiler has no way of knowing the size that will be stored in that location compile time as it could hold different types

