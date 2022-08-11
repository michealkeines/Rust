For all x86 arch

VGA Text buffers which is direclty connected to the screen is accessible via memory-mapped I/O to the address 0xb8000

![[Pasted image 20220730220002.png]]

This means that read and writes to that address (0xb8000) don't access the RAM, but direclty the text buffer on the VGA Hardware

Memory Mapped I/O - we have shared access to a Physical address of I/O devices
Port Mapped I/O

They use sperate I/O bus for communication, each connected peripheral has one or more port numbers. To communicate with such an I/O port there are special CPU instructions called in and out, which take a port number and a data type 

