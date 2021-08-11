Setup:

curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env

Anatomy of rust:

`fn main() {
	println!("Hello, world!");
}`
	
Main is the first code that will be called when a program is executed.

Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having rust installed

using cargo as package manager

create a new project 

`cargo new <name of the project>`

any dependencies can be added in the Cargo.toml file under [dependencies]


we can build a project using cargo build
we can build and run a project using cargo run
we can check for errors using cargo check

excutables are stored under target dir

