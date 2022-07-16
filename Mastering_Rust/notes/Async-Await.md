# Asynchronous Rust

Why Async? It depends on what is that we are trying to write, If your example function is waiting to download a 10mb file from a server

```
fn get_file() -> Vec<u8> {
	// request to server and recv 10mb of raw bytes
}
```

If we call get_file() function from the main function, it will block the whole main thread

```

fn get_file() -> Vec<u8> {
	// request to server and recv 10mb of raw bytes
}

fn extra_bytes() -> Vec<u8> {
	// request to serverr and recv 40mb of raw bytes
}

fn main() {
	let bytes = get_file(); // this is gonna take a while
	let extra_bytes = get_extra(); // this is also gonna take a while
	let convert_string = String::from_utf8(&bytes).unwarp(); // gonna be a while before we reach this line
	
}
```

Lets say get_file is gonna take 0.3 sec, and get_extra is gonna take 0.5, to finish the whole process, the main function is gonna take 0.8 sec to continue execution

As get_file() and get_extra() are not dependent on each other's completion, we could spawn them in threads (old threading method)

but we won't have any information about the thread's current status or progress

## Future

Future Trait is like an place holder for the result of an operation


```
trait Future {
	type Ouput;
	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```


Asynchronous programming gives us an abstraction over an task bu implementing the Future trait, that allows us a pause and start the task  untill it gets done.

we also need someone to check on the Future for updates, an executor.

The Future can set when to start them again.

so if we create two futures get_file() and extra_bytes(), and our executor will have two futures and it will poll the Future which is set to start now

if the Future doesnt set when to start it, the executor wont know if starting it would make any progress.

thus with one thread we are able to get the illusion of the both futures running asynchronosly


Future Trait will define the concrete type that will be returned as the result of the operation that is implementing this trait

poll method defines how the current status will be sent back, It returns an Enum Poll

```
enum Poll<T> {
	Ready(T),
	Pending
}
```

Ready(Output) -> if the result of the operation is ready
Pending -> if the operation is still going on



