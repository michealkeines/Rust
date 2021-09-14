Arrays are efficient because they are a fixed size container of length n, where every element to jump to using the simple formula
	value = startaddress + index * elment size
	
a Vec<T> is stored in stack as 
	
	buf : pointer to buffer
	cap: total element
	length: current elements present
	

	