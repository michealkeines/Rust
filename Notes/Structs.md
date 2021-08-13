structs can contains any data types and can be accessed as key: value pairs 

all fields should be mutable, we can have anything immutable

but you can make the whole struct as mutable or immutable as per  your need

Struct User {
	name: String,
	id: u32,
	active: bool
}

we can have struct tuple, it was way to name tupbles

struct red(i32,i32,i32);
struct green(i32,i32,i32);

each of it can be accessed with its name.

to define functions inside a struct we use impl keyours with struct name

all the functions should take self as the first paramter that hold the current struct instance

struct user {
	user: String,
	id: u32,
	active: bool
}

imp user {
	fn isActive(&self) -> bool {
		self.active	
	}
}

here &self is to send the object without tranfering the ownership
&mut self will make its a mutable referece that transfers ownership

we can also declare function inside impl without self, whihc makes them associated functions

it allows us to namespace functionality that is particular to your struct without having an instance available.

they can directly be accessed with object::function()

