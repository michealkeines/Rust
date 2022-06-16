Important differences

![[Pasted image 20220430124644.png]]


represent differnt notations


![[Pasted image 20220430235602.png]]

![[Pasted image 20220501000129.png]]

Traits methods can only be acced if the trait is within the scope

![[Pasted image 20220501000623.png]]

dont compare floating point values

![[Pasted image 20220501001044.png]]


![[Pasted image 20220501002809.png]]


![[Pasted image 20220501011034.png]]

manually indexing colection has runtime costs

test[i]

![[Pasted image 20220501011316.png]]



![[Pasted image 20220509094301.png]]

! makes the function never return


if the variable is not a reference to make it mutable

we can able mut keywork infront of the name

fn test(mut val: String) // wihout referece

fn test(val: &mut String) // with refeerence


Floating Points

![[Pasted image 20220512085821.png]]

![[Pasted image 20220512085851.png]]

![[Pasted image 20220516025917.png]]

CELL and REFCELL

Virtual Memory:

Word: This corresponds to width of the CPU's registers

Page: A fixed size block of words of real Memory, Typically 4 KB in size for 64bit OS


Page Fault: An error raised by the CPU when an valid memory address is requested that is not currently in the physical RAM, this signals to the OS that at least one page must be swapped back into memory

Virutal Memory: The programs view of it's Memory, All data accessible to a program is provided in tis address space by the os

Real Memory: The OS's view of the physical memory available on the system.

Page table: The data structure maintained by the OS to manage translating from virtual to real memory

Segment: A block within virtual memory, Virtual memory is divided into blocks to minimize the space required to translate between virtual and physical addresses

Segmentation fault: An error raised by the CPY when an illegal memory address is requested

MMU: A component of the CPU that manages memory address translation. Maintains a cache of recently translated addresses called TLB, that is translation lookaside buffer.

Virutal memory optimization to use cache

![[Pasted image 20220516065404.png]]

![[Pasted image 20220516065700.png]]

![[Pasted image 20220528012318.png]]

Trait objects ownership

![[Pasted image 20220528024947.png]]

![[Pasted image 20220604073050.png]]

Time NTP

![[Pasted image 20220604073104.png]]

Thread closure lifetimes

![[Pasted image 20220605020613.png]]

![[Pasted image 20220605021512.png]]

join make the calliing thread to wait till its child threads finishes execution

![[Pasted image 20220605023024.png]]

yeild signals the os to stop sheduling the current thread util futher notice

![[Pasted image 20220605025406.png]]

as per channels, we create a two objects transmitor and reciever

we call tx.send() to send something to our respective reciver whihc will wait using recv() call

design to crate connect bettwen two points

sender_tx, sender_rx

receiver_tx, receiver_rx

we send the messages to sender_rx.send(), within a thread sender_rx.recv() will receive all those messages and pass it to receiver_tx.send(), and receiver_rx might be waiting for the reciver_rx in main thread or in other thread