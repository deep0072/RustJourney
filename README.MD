# RUST tutorial

## to run .rs file we can use two method
1. ``rstc "rustfile" ``
2. ``cargo run`` it will work only when project created using cargo new "project name"


# OwnerShip 

### Ownership is a set of rules that govern how a Rust program manages memory.

### first we need to understand  about the Heap and Stack

#### Stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways.
#### Stack ==> All data stored on the stack must have a known, fixed size. 
#### Heap ==> Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

### Now lets Understand the process of Allocation how its done

#### The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating)
#### Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer

#### Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation

### SO how function and its data managed in memory 

#### When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack


### Key Points:

1. Stack is fast, LIFO (Last-In, First-Out), and organized.
2. Heap is slower, flexible, and larger.
3. Functions and their data are managed on the stack.
4. Dynamically allocated data is managed on the heap.
5. Pointers on the stack link to data on the heap.









