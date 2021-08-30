iteratinf over each item of a list or sequence

iterators are lazy, they are not consumed util they are actuall used

let a = vec![2,3,4,5];

let a_iter = a.iter(); // in this step nothing will be created

for val in a_iter { // only when the iter is actually used the list is consumed.
}

if want a iterator that take ownership of the list then we use into_iter()
if want want mutable references then we call iter_mut()


iter has many inbuilt methods like sum(), map(), filter()

sum() -> adds all values to return the total sum
map(<closure>) -> passes every element in the list to a closure and places the result in a vector list
filter(<closure>) -> passes every element in the list to a closure and places only the result that are true, here the closure should return a true or false.
	
any object can implement itarator trait, which has one function that is the next()
	
let sum: u32 = counter::new().
        zip(counter::new().skip(1)).
        map(|(a,b)| a*b).
        filter(|s| (s % 2) == 0).
        sum();
    println!("{}",sum);
	
iterators and closure have the same or slightly better performance than normal implementation using any loops
	

