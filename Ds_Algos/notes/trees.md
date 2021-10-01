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

how many red node can there be if a tree is balanced,

as a red node can only come from a black node, there will be at most as many as there are blacknodes.

the longest path (root to farthest NIL) is no more than twice the length of the shortest path (root to nearest NIL)

-> shortest path: all black nodes
-> largest path: alternating red and black

rotation:

-> alters the structure of a tree by rearranging subtrees
-> goal is to decrease the height of the tree
	- red-black trees: maximum height of O(log n)
	- larger subtrees up, smaller subtrees down
-> does not affect the order of elements, ie left will be smaller and left will be greater values

![[Pasted image 20210915061047.png]]

![[Pasted image 20210915061224.png]]



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


tries:
	here all the strings could be stored as chars to root node, like ABC ABD 
									a
								  /
								b
							 /	   \
						   d		c	
this way any string can be saved and takes by traversing the tree


B-tree:
	it can have varying amounts of those key-value pairs,
	they will have maximum number of children defined by the order parameter.
	
Rules:
	each node can only ahve order children
	each not that is not a left node or root has at least order/2 childern
	the root node has at least two children
	all nodes hold order - 1 keys when they have order children
	all leaf nodes apper on the same level
	
self balancing this tree works same way as  red-black tree, 

new keys can only be inserted at the leaf level.
once the new key has found a node, the node is evaluated to preceding rules, in particular, if there are now more than order - 1 keys, if that is the case, the node has to be split, moving the center key to the parent node

search and insert take o(log(n)) with the reblancing



