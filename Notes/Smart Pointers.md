A pointer si a general concept for a variable that contains an address in memory.

Smart pointers are data structures that not only act like a  pointer but alse have additional metadata and capabilities.

in rust String and Vec<T> are both examples of smartpointers because they own some memory and allow you to manipulate it. they have metadata and extra capabilities.
	
Smat pointer are usually implemented using structs, the characteristic that distinguishes a smart pointer from an oridinary struct is that 
	
	smart pointers implement the deref and drop trait

Deref trait allowsa instance of the smart pointer struct to behave like a reference so you can write code that works with either instance of the pointer or smart pointers.
	
Drop trait allows you to customize the code that it run when a instance of the smart pointer goes out of scope.
	
examples of common smart pointers
	
Box<T> for allocating values on the heap
Rc<T> a reference counting type that enables mutiple ownership
Ref<T> and RefMut<T> accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time.

fn main() {
	let a = Box::new(3); // its is allocated in heap and the pointer is stored instack
	println!("{}",a);
}
	
this is usefull to store recursive types, whose size can't ne known in compile time.
	
so we can  place such types inside box to make it possible
	
enum list {
	cons(i32, Box<List>),
	Nil
}
	
this is just cons(x,y) whihc is a recursive holding 
	
	because a Box<T> is a pointer, rust always knows how much space a Box<T> needs: a pointers size doesnt change based on the amount of dat its pointing to.
	
Deref coercion is a convenience that rust performs on arguments to functions and methods. 
deref coercion works only on types that implement the deref trait.
	
deref coercion converts such a type into a reference to anothr type. for example, deref coercion happends automatically when we pass a reference to another type.
	
deref coercion can covert &string to &str because String implements the Dered trait such that it returns str.

fn hello(val: &str) {
    println!("{}",val);
}


fn main() {
    let y = String::from("test");
    hello(&y);
}
	
here we pass a reference of string to a method that accepts &str, rust will automatically convert it to str, as string implements deref trait.

Drop trait is called automatically when a something gets out of scope.
	
we can define custom drop traits for structs or smart pointers by implemting drop trait and drop method.
	
rust owership makes use that a var is only dropped
	
we can manually drop a variable us ing drop(var);
	
Rc<T> Reference counting keeps track of references to a values which determines whether or not a value is still in use, if there are zero references to a value, the value can be cleaned up without any references becoming invalid.
	
Rc<T> is used when we want to allocate some data on the heap for mutiple parts of our program to read and we can't determine at compile time which part will finish using the data last.

it can only be used in single-threaded scenarios.
	
RefCell<T> type has single owership
	
it is usefull when you're sure your code follows the borrowing rules but the compiler is unable to understand and guarntee that.
	
it is only for use in single threaded mode
	
Reasons to choose Box<T>, Rc<T> or RefCell<T>:
	
	-> Rc<T> enables multiple owners of the same data, Box<T> and RefCell<T> have single owners.
	-> Box<T> allows immutable or mutable borrows checked at compile time. Rc<T> allows only immutable borrows checked at compile time. RefCell<T> allows immutable or mutable borrows checked at runtime.
	-> Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
	
Mutating the value inside an immutable value is the interior mutability pattern.
	

pub trait Messenger {
    fn send(&self, msg: &str);
}
	
if we want to create messenger trait object, it should only have immutable reference values
	
struct MockMessenger {
        sent_messages: Vec<String>,
    }

	if we try to place this struct and impletent the send method and update that sent_messges vector, we will need a mutable self, which will be against the traits signature,
	
	this is the right situation for RefCell<T> istead of changing the function signature, update the Mockmesaage vector with 
	
struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

	now this wont violate the immutablility and we can still mutatate the sent_messages vector
	
fn send(&self, message: &str) {
	self.sent_messages.borrow_mut().push(String::from(message));
}

in normal reference we use & and &mut to make immutable and mutable reference
in RefCell we use borrow() and borrow_mut() to make immutable and mutable reference

	if it should follow these same rule,
	-> there can be several immutable reference, it will keep a count of all reference
	-> there can only be one mutable reference

it will panic in the runtime if the code violated these rules
	
	let mut one = self.sent_messages.borrow_mut();
	let mut two = self.sent_messages.borrow_mut();

	one.push("test");
	two.push("what");
	
this code will panic as there are two mutable borrow
	
we can use RefCell<T> and Rc<T> in combination to get have serveral mutable reference of some data, but the sigunature still stays stay immutable 
	
by using RefCell<T> we have outwardly immutable List value, and we use the methods on RefCell<T> that provide access to its interior mutability so we can modify our data when we need to.
