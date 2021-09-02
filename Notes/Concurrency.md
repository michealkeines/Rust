Concurrent programming is where different parts of a program execute independently

Paralled programmin is where different parts of a program execute at the same time.

in most current operating systems, an executed program's code is run in a process, and the operating system manages mutiple processes at once, within your program, you can also have independent parts that run simultaneouslly. The feature that run these independent parts are called threads.

Rust model calls the operating system APIs to create threads is sometimes called as 1:1, meaning one operating system thread per one language thread.

thread::spawn() // new thread
thread::sleep() // stop a thread for given time

to make sure all the thread run completely we call join method in the return value of thread::spawn() which is a join handle

let handle = thread::spawn(|| <some closure here>);
	
handle.join.unwrap(); // waits it all the threads finish execution
	
the main thread will be in waiting state, so where join is called, can affect whether or not your threads run at the same time.
	
to use a value that is declare in the main thread, we have use move keyword in our thread closure, thus the thread will take the owership of the variable

let v = vec![1,2,34];
	
let handle = thread::spawn(move || {
	println!("{:?}", v); // in this case owenship of v is moved to the thread	
});
// v cant be used in main thread anymore

Message Passing:
	one increasingly popular approach to ensuring safe concurrency is message passing, where threads communicate by sending each other messages containing data
	
One major tool rust has for accomplishing message-sending concurrency is the channel

A channel in programming has two halves: a transmitter and a receiver.
	
one part of your code calles method on the transmitter wtith the dat you want to send and another part checks the receiving end for arriving messages.
	
a channel is send to be closed if either the transmitter or the receiver is dropped.

rust standard lib implements a mpsc stands for mutiple producer and single consumer.

there can be many senders but only one receiver
	
let (tx, rx) = mpsc::channel(); // creates a new channel the return a tuple with transmiter(tx) and receiver(rx).

tx has send method to send anything down the stream
	
rx is iterator, which has
	recv -> the thread waits till it receives something or ends if the transmiter is dropped
	try_recv ->  returns message or err immedeatly, dont block the thread

	rx can be used in for loop and kept open for receiving futher messages, the loop will end when the transmiter is dropped
	
to create mutiple producers we can just clone the tx and send different messages in the cloned instances, which will be also be received by the rx
	
let (tx, rx) = channel::mpsc();

	t1 = tx.clone();
	t2 = tx.clone();

Mutex
	
Shared memory concurrency is like mutiple ownership, mutiple threads can access the same memory location at the same time.
	
mutex is an abbreviation for mutal exclusion, a mutex allows only one thread to access some data at a given time. 
	
to access the data in mutex, a thread must first signal that it wants access by asking to acquire the mutex lock.
	
the lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data
	
therefore the mutex is described as gaurding the data it hold via the locking system.

	
rules for mutex:
	-> you must attempt to acquire the before using the data
	-> you must unlock the data so other threads can acqire the lock

let a = Mutex::new(5);

every mutex is type mutex<T>, they implement the defer trait thus the T can be accessed by deferencing them
	
this is would like mutex<i32> -> to use the i32 value we have to acquire the lock and dereference to access the internal value

let mut m = a.lock.unwrap();
*m = 4;

the lock is automatically release with the drop trait of lock, thus we wont have to unlock it manually

but to have mutiple threads access a single var, we have to use Reference counting warpper, but Rc<T> is not thread safe, Rust provides Arc<T> which is thread safe

Send Trait is implement by the types that are allowed in multi threaded 
context
	
Sync Trait is make the type references in multiple threads
	

	
	
	
	

	
