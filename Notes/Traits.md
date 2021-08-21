A Trait tells the rust compiler about functionality a particular type has and can share with other types.

we can use traits to define shared behavior in a abstract way.

Traits are similar to a feature often called interfaces in other language, although with some differences.

if we have two structs that needs to have common function we can define that as a trait and implement that trait accordingly

we can implement a trait on a type only if the either the trait or type is local to our crate.

we cant implement external traits on external types.

this rules ensures that ohter people's code cant break yours and vice versa. without this rule, two crates could implement the same trait for the same type and rust wouldnt know which implementation to use.

we can give give default implemenation for trait methods that can be called, and overrding a default implemenation is same as implementing without default method.

`
pub trait Summary {
    //Default method
    fn loading(&self) -> String { 
        format!("Loading content")
    }
    //should be implemented by the struct that wants this trait
    fn summarize(&self)  -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,                                               
    pub content: String                                               
}                                                                     
                                                                      
impl Summary for NewsArticle {                                        
    //implemention for trait                                          
    fn summarize(&self) -> String {                                   
        format!("{}, by {} ({})", self.headline, self.author, self.location)                                                                
    }                                                                 
}                                                                     
                                                                      
pub struct Tweet {                                                    
    pub username: String,                                             
    pub content: String,                                              
    pub reply: bool,                                                  
    pub retweet: bool                                                 
}                                                                     

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tw = Tweet {
        username: String::from("micheal"),
        content: String::from("this is me"),
        reply: true,
        retweet: false
    };
    println!("{}", tw.loading());
    println!("{}", tw.summarize());
}



`
we can pass objects of two class that implement a trait by defining a function that takes type of trait name, thus it will accept any object that implements the specified trait.

`pub fn notify(item: &impl Summary) {
	println!("{}", item.summarize());
}`

short syntax for this is

`pub fn notify<T: Summary> (item: &T){

}`

we can specify mutiple trait founds with the + syntax

that we only accept the object that has implemented all the traits that are specified.

`pub fn nofity<T: Summary + Display>(item: &T) {

}`

only the objects that ahve implemented both summary and dispaly can be passed to this funciton.

we can use where to specify the traits in a better way that is easy to read

`fn some_function<T, U>(t: &T, u: &U) -> i32 
	where T: Display + Clone, U: Clone + Debug
{

}`

we cam return a impl trat object thus, the return value can be any object that implemnts the given trait

