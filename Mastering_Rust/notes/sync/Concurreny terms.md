A small glossary of terms relating to concurrency

This subfield is filled with jargon. Here is a brief introduction to some important terms and how we use them:

•Program—A program, or application, is a brand name. It’s a name that we use to refer to a software package. When we execute a program, the OS creates a process.

•Executable—A file that can be loaded into memory and then run. Running an executable means creating a process and a thread for it, then changing the CPU’s instruction pointer to the first instruction of the executable.

•Task—This chapter uses the term task in an abstract sense. Its meaning shifts as the level of abstraction changes:

a. When discussing processes, a task is one of the process’s threads.

b. When referring to a thread, a task might be a function call.

c. When referring to an OS, a task might be a running program, which might be comprised of multiple processes.

•Process—Running programs execute as processes. A process has its own virtual address space, at least one thread, and lots of bookkeeping managed by the OS. File descriptors, environment variables, and scheduling priorities are managed per process. A process has a virtual address space, executable code, open handles to system objects, a security context, a unique process identifier, environment variables, a priority class, minimum and maximum working set sizes, and at least one thread of execution.

•Each process is started with a single thread, often called the primary thread, but can create additional threads from any of its threads. Running programs begin their life as a single process, but it isn’t uncommon to spawn subprocesses to do the work.

•Thread—The thread metaphor is used to hint that multiple threads can work together as a whole.

•Thread of execution—A sequence of CPU instructions that appear in serial. Multiple threads can run concurrently, but instructions within the sequence are intended to be executed one after another.

•Coroutine—Also known as fibre, greenthread, and lightweightthread, a coroutine indicates tasks that switch within a thread. Switching between tasks becomes the responsibility of the program itself, rather than the OS. Two theoretical concepts are important to distinguish:

a. Concurrency, which is multiple tasks of any level of abstraction running at the same time

b. Parallelism, which is multiple threads executing on multiple CPUs at the same time

Outside of the fundamental terminology, there are also interrelated terms that appear frequently: asynchronous programming and non-blocking I/O. Many operating systems provide non-blocking I/O facilities, where data from multiple sockets is batched into queues and periodically polled as a group. Here are the definitions for these:

•Non-blocking I/O—Normally a thread is unscheduled when it asks for data from I/O devices like the network. The thread is marked as blocked, while it waits for data to arrive.

•When programming with non-blocking I/O, the thread can continue executing even while it waits for data. But there is a contradiction. How can a thread continue to execute if it doesn’t have any input data to process? The answer lies in asynchronous programming.

•Asynchronous programming—Asynchronous programming describes programming for cases where the control flow is not predetermined. Instead, events outside the control of the program itself impact the sequence of what is executed. Those events are typically related to I/O, such as a device driver signalling that it is ready, or are related to functions returning in another thread.

•The asynchronous programming model is typically more complicated for the developer, but results in a faster runtime for I/O-heavy workloads. Speed increases because there are fewer system calls. This implies fewer context switches between the user space and the kernel space.

.5.1 Threads

A thread is the lowest level of isolation that an OS understands. The OS can schedule threads. Smaller forms of concurrency are invisible to the OS. You may have encountered terms such as coroutines, fibers, and green threads.

Switching between tasks here is managed by the process itself. The OS is ignorant of the fact that a program is processing multiple tasks. For threads and other forms of concurrency, context switching is required.
.5.2 What is a context switch?

Switching between tasks at the same level of virtualization is known as a context switch. For threads to switch, CPU registers need to be cleared, CPU caches might need to be flushed, and variables within the OS need to be reset. As isolation increases, so does the cost of the context switch.

CPUs can only execute instructions in serial. To do more than one task, a computer, for example, needs to be able to press the Save Game button, switch to a new task, and resume at that task’s saved spot. The CPU is save scum.

Why is the CPU constantly switching tasks? Because it has so much time available. Programs often need to access data from memory, disk, or the network. Because waiting for data is incredibly slow, there’s often sufficient time to do something else in the meantime.
You missed out on some activities - why not   try them now?
.5.3 Processes

Threads exist within a process. The distinguishing characteristic of a process is that its memory is independent from other processes. The OS, in conjunction with the CPU, protects a process’s memory from all others.

To share data between processes, Rust channels and data protected by Arc<Mutex<_>> won’t suffice. You need some support from the OS. For this, reusing network sockets is common. Most operating systems provide specialized forms of interprocess communication (IPC), which are faster, while being less portable. 


.5.4 WebAssembly

WebAssembly (Wasm) is interesting because it is an attempt at isolating tasks within the process boundary itself. It’s impossible for tasks running inside a Wasm module to access memory available to other tasks. Originating in web browsers, Wasm treats all code as potentially hostile. If you use third-party dependencies, it’s likely that you haven’t verified the behavior of all of the code that your process executes.

In a sense, Wasm modules are given access to address spaces within your process’s address space. Wasm address spaces are called linear memory. Runtime interprets any request for data within linear memory and makes its own request to the actual virtual memory. Code within the Wasm module is unaware of any memory addresses that the process has access to.
You missed out on some activities - why not   try them now?
.5.5 Containers

Containers are extensions to processes with further isolation provided by the OS. Processes share the same filesystem, whereas containers have a filesystem created for them. The same is true for other resources, such as the network. Rather than address space, the term used for protections covering these other resources is namespaces.
.5.6 Why use an operating system (OS) at all?

It’s possible to run an application as its own OS. Chapter 11 provides one implementation. The general term for an application that runs without an OS is to describe it as freestanding—freestanding in the sense that it does not require the support of an OS. Freestanding binaries are used by embedded software developers when there is no OS to rely on.

Using freestanding binaries can involve significant limitations, though. Without an OS, applications no longer have virtual memory or multithreading. All of those concerns become your application’s concerns. To reach a middle ground, it is possible to compile a unikernel. A unikernel is a minimal OS paired with a single application. The compilation process strips out everything from the OS that isn’t used by the application that’s being deployed. 
