newtype is a pattern that can used to get around some restrictions 

like vec<T> can implement ftm::display trait as to implement a trait either the type or trait should be local to our carte
	
the cant implent dispay trait which is not in our local carte to vec<T> type which is also not in out local crate, to overcome this we add a warpper without any runtime penalty
	
like a tuple struct with vec<T>

	struct test(vec<String>);
	
	this struct hold a vec<string> so we have this type locally so we can implement display trait in it
	
	this also makes a new encapsulation method,
	
	instead of exposing a type directly we warp it in a tuple struct and impl the functions that we might need to explose, making it usefull and resrticted as we want
	
	struct millimeter(u32);
	struct meters(u32);
	
	fn test(val: millimeter) {}
	
	here we have two tuple structs both hold u32, but we have seperation with name, thus if a fun takes milimeters we cant couldnt compile accidently with meters makeing it perfect for abstration of type custom type checking
	
	this programming pattern is called newtype pattern
	
	this is a light weight way to achieve encapsulation to hide implementation details
	
	Another type of pattern is type aliases
	
	type meters = u32;
	
	we can use meters as u32, 
	
	let a: u32 = 5;
	let c: meters = 3;
	
	println!("add : {}", a+c);
	
	this way we can add u32 + meters because meters is just u32
	
	this doesnt help in type checking like newtype, this alias helps in reducing the amount of repetaiton of names
	
	thus lenghty custom types can be aliaed to simple readble names and make code readbility better
	
	like 
	
	let f: Box<dyn fn() + Send + 'Static> = Box::new(|| prinln!("test"));
		
	Box<dyn fn() + Send + 'Static>  is a big type
	
	we alias that to
	
	type customtype = Box<dyn fn() + Send + 'Static>;
		
	let f: customtype = Box::new(|| println!("test"));
	'	
	this makes it easier to read and write code
	
	we can customize more by creating genrics like alias
		
	there are different variants of result 
		
	type Reslt<T> = std::result::Result<T, std::io::error>;
		
	this was if call Result<usize> it will replaced in std::result::Result<usize, std::io::error>;
	
	Rust has Never type whihc is denoted by `!`

	fn test -> ! {
	}
	
	this means test function never returns, it is usefull in pattern matching
	
	like 
	
	let guess: u32 = match gess {
		Ok(num) => num,
		Err(_) => "not correct"
	}
	
	this will error out, as all the arms of match should return same type, so we can use ! never type,
	
	let guess: u32 = match gess {
		Ok(num) => num,
		Err(_) => continue
	}
	
	continue return never type, that means the compiler will infer the type from other arm, u32 in this case
	
	the formal way of describing this behaviour is that expressions of type ! can be coerced into any other type.
	
	in rust we need to know what is the size of type we use,
	
	to have dynaimc size type, we put the type behind a pointer
	
	even generic traits use that
	 
	fn test<T>(t: T) {
	
	}
	
	traits like this who type size can be known at compile time implemnt sized in bakcground like this 
	
	the above code will be converted to
	
	fn test<T: Sized>(t: T) {
	}
	
	Sized is added by compiler
	
	if the size is unknown we can use ?Sized
	
	thus T is changed to &T, reference to a type 
	
	fn test<T: ?Sized>(t: &T) {
	
	}
	
	