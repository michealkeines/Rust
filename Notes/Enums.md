we can create custom type using enum

enum ip {
	v4(string),
	v6(string)
}

we can make them hold string value

we and add function with impl like structs, 

the advantage is that we can use this to make custom type that could be enumurated to pick one

this is important so that we can make function that take this custom type and it can hold any value that is defined within the enum

we can use option<T> enum which can either take a specified some(T) or None

this helps us with assiging a variable is null as of this moment, but might containt something later, whihc will be of this <T> type
	
