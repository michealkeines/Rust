we can pass functions as agruments to another function using a funciton pointer

fn call(u: u32) -> u32 {
	u+ u
}

fn test(ptr: fn(u32) -> u32, arg: u32) -> u32 {
	ptr(arg)
}

we have argument ptr which takes a function pointer, we have difine the function's argument type and return type if any.

we can return closures from a function

fn return_closure() -> dyn Fn(i32) -> i32 {
	|x| x+1
} // this wont work as we dont know the size of the returned value here

we should just directly return  a closure as the size it might not be known, its not possible, so we can just wrap it in a pointer type

fn return_closure() -> Box<dyn Fn(i32) -> i32> {
	Box::new(|x| x+1)
} // we know the size of box pointer that will contain the closure

