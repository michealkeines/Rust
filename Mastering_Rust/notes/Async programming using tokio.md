Future

It is essentially a placeholder for the result of an operation.

as you would expect, the result of an operation can be in one of two states

-> either the operation is still in progress and thre result is not available yet

-> the operation is finished and the result is available (could also be an error)



any type can implement this Future Trait to act like a future

![[Pasted image 20220706122353.png]]

An type must specify the item and error and also implement the poll methof that gets the current state of the computation

![[Pasted image 20220706122708.png]]

