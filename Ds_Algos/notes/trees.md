Tree data structure consist of a root node with branches and leaves

binary tree - in this the nodes of the tree are arranged in a way that anything to left to the root is lower than root value and right is higher than root value

inserting a new node is made by walking through the tree and finding the correct spot to place the value to insert

self balancing tree also called as red-black tree

red-black tree satisfies a set of rules:

-> the root node is always black
-> each other node is either red or black
-> all leaves are considered black
-> a red node can only have children
-> any path from the root to its leaves has the same number of black nodes, also known as black-depth

https://brilliant.org/wiki/red-black-tree/

any black node will have 

no of pointers for any black node = 2*(r+2)

where,
	r is the number of red nodes
	
case1:
	the node weve inserted is red and its parent sibling is red, we know that the inserted node's grandparent wil be black, so all we need to do is switch the coloring of the insterted node's grandparent with cloring of the inserted node's parent and parent's sibling.
	
![[Pasted image 20210914132751.png]]

case2:
	when the node's parent is red, but the parent's sibling is black and node's values is between those of its parent and grandparent.
	we handle case 2 by performaing a rotaiton that takes us to case3, there are two kinds of rotation, a left rotation and a right rotation, in case 2 we use left rotation
	
![[Pasted image 20210914141207.png]]

case3:
	it involves a right rotation on the grandparent. swap the node's parent with the grandparent and move the parent to the right of the grandparnet nad move the node to left of the grandparent
	
![[Pasted image 20210914142058.png]]



