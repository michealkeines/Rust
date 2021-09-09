traits can take return item type as type and compare with type as rhs

pub trait add<Rhs=millimeter> {
	type output;
	fn add(self, rhs: Rhs) -> Self::output;
}

impl add<Rhs=millimeter> for test {
	type output = meters;
	fn add(self, rhs: meters) -> Self::Output {
		//do something here
	}
}


opertor overloading can be achiveved this way, this we can add millimeters to self and get meter as output

we specifiy the rhs as object to add with millimeter
we specify the result type in type meter

by default it Rhs will be self, thus it can be used to add a object with similar type

impl add for test {
	type output = meters;
	fn add(self, rhs: meters) -> meters {
	}
}

if a struct implement two traits and both have same method name in it, we have to explicitly call the functions to call the specific method


impl pilot for human {
	fn fly(self) {
	}
}


impl wizard for human {
	fn fly(self) {
	}
}

impl human {
	fn fly(self) {
	}
}

this human struct has three different versions of fly to call them specifically

let person = human;
person.fly(); // calls the fly method directly implemented in human

pilot::fly(&human); // calls the fly method in pilot trait

wizard::fly(&human); // calls the fly method in wizard trait

this works because those methods take &self as an argument

but associated functions that doesnt take self wont work that way

trait animal {
	fn test() {
	}
}

struct dog {};

impl dog {
	fn test() {
	}
}

impl animal for dog {
	fn test() {}
}

here test is a associate funtion

so here calling 

let t = dog;

animal::test(&t); // this wont work 

to call the method implemented udner animal we have explicilty annotate the type

<dog as animal>::test();
	
this indicated we want to treat the dog as animal thus we atn to call method test method from animal trait
	
we can make trait take another trait this way we enfore that both traits should be implemented if want to compile
	
trait one: zero {}
	
struct test {};
	
impl one for test {}; // if we compile with is one implementation,we will get error taht we need to implement both one and zero traits
	

	
	
