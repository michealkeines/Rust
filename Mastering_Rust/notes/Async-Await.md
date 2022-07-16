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


Asynchronous programming gives us an abstraction over an task by implementing the Future trait, that allows the task to yeild when it is blocked and add itself back to the exection when ready

we also need someone to check on the Future for updates when the Future beleives it is ready now.

An executor helps with the update check.

so if we create two futures get_file() and extra_bytes(), and our executor will have two futures and poll will be called by the the executor on those futtures

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


Executor is just a queue, Futures are added to the que and executor will call the poll method on those futures

Let's say if we add two jobs to the Executor queue

To start the job, Executor will call Poll method in all the jobs in the queue

every Future will have it's own implemenation of when it want itself to be added to the executor queue again.

In Rust word,

Executor will be a Receiver Stream and user adds the Futures into tthe stream to start the operation.

Example:

![[Pasted image 20220716213622.png]]

TimerFuture is the job that needs to be run

![[Pasted image 20220716213713.png]]

It implements the Future Trait, everytime when the executor call this poll method, it will check if the shared_state is completed or it will clone the waker passed by the executor to push itself again into the stream when it is ready


![[Pasted image 20220716214002.png]]

The job in the Future is very simple, it sleeps for the duration that is passed and calls the wake method in the waker that it cloned when the poll was initially called for this Future

wake method will add this Future again to the executor stream and this time when the executor calls poll, the job will completed with ready state.

