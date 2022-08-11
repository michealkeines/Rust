https://stackoverflow.com/questions/40302026/what-does-the-tt-metavariable-type-mean-in-rust-macros

![[Pasted image 20220731130614.png]]


```
macro_rules! {
	(fn $name:ident $params:tt $body:tt) => {}
}
```

this would match 

```
fn main() {
	println!("test");
}
```

$name => main

$params => ()

$body => { println!("Hello world!"); }



tt means token tree, it matches anything 

![[Pasted image 20220731132223.png]]

