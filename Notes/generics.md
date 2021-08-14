with `<T>` you can create funtions that could potentially take different data types
	
`fn largest<T>(list: &[T]) -> T`
	
this way this function can take int, float or any appropriate types
	
we can add two types using 
	
	struct test<T, U> {
		x: T,
		y: U
	}
	
	this can the stuct can contain differnt data types
	
struct val<A,B, V> {
    x: A,
    y: B,
    z: V
}

fn main() {
    let a = val{x: 1, y:1, z:2 };
    let b = val{x: String::from("test"), y: 4.5, z:String::from("omg")};
    println!("{}", b.z);
}
	
methods in structs can also take generics
	
impl`<T> `// this means we are defining a method for all possible datatypes that might call this method
	
to add method for specific type of data
	
`imp val<u32> {
	fn decima() {
	}
}`
	
thus lets us define seprate method for differnt types
	
	
`struct val<A,B, V> {
    x: A,
    y: B,
    z: V
}

impl<A: std::fmt::Debug,B,V> val<A,B,V> {
    fn test_all(&self) {
        println!("This is maddness {:?}", self.x);
    }
}

impl val<String, f32, String> {
    fn test(&self) {
        println!("good one");
    }
}

fn main() {
    let a = val{x: 1, y:1, z:2 };
    let b = val{x: String::from("test"), y: 4.5, z:String::from("omg")};
    a.test_all();
    b.test_all();
    println!("{}", b.z);
}`
	
	
	