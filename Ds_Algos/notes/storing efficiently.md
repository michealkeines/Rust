
Rust with its ownership model calls for considering lifetimes, memory placement, and mutability in their algorithmic design.

-> trade offs considering speed and readability
-> Accessing heap and stack variables
-> how immutablility influences design


Rust types allow for zero overhead structures, so no additional metadata is stored

Every time a pointer of some kind(&, Rc, Box, Cell) is created, the reference is stored alongside the length and some metadata.


