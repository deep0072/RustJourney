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


## so now comeback to Ownership Transfer:



#### When you pass a variable to a function as a parameter, ownership of that variable is typically transferred to the function. This means the function takes control of the variable's memory and is responsible for its lifetime.
### The original variable in the calling scope becomes invalid and cannot be used again.

code example 

```
fn greet(name: String) {
    println!("Hello, {}!", name); // Use the new name within the function
}

fn main() {
    let my_name = String::from("Alice");
    greet(my_name); // my_name is moved to the greet function
    // println!("My name is {}", my_name); // This would error: my_name is no longer valid here
}
```


# Borrowing

## passing in refrences as function params is known as borrowing. means when we pass variable as params in function then it wont drop even after execution of function 



```
fn main() {
    let s1 = String::from("Deepak");

    let length = get_length(&s1); //& denote the reference

    println!("{}", s1); // it still working because we are passing refrence in function. so this is the borrowing;
}

fn get_length(s:&String) -> usize {
    let size = s.len();
    size
}

```



# Rust Structs Guide

This guide will explain how to use structs in Rust, a programming language that emphasizes safety and performance. 

## Defining Structs

Structs in Rust are similar to tuples, in that they can hold multiple related values. However, unlike tuples, structs require you to name each piece of data, making it clear what the values represent [Source 0](https://doc.rust-lang.org/book/ch05-01-defining-structs.html). 

Here's an example of defining a struct:

```rust
#[derive(Debug)]
struct User {
   active: bool,
   user_name: String,
   email: String,
   sign_in_count: u64,
}
```

In this example, `User` is a struct with four fields: `active`, `user_name`, `email`, and `sign_in_count`. The `#[derive(Debug)]` annotation above the struct declaration is used to enable formatting of the struct for output [Source 1](https://doc.rust-lang.org/std/keyword.struct.html).

## Creating Instances of Structs

After defining a struct, you can create an instance of it by specifying concrete values for each of the fields. Here's an example:

```rust
let user1 = User {
   active: true,
   email: String::from("deepak@gmail.com"),
   user_name: String::from("Deepak"),
   sign_in_count: 1,
};
```

In this example, `user1` is an instance of the `User` struct [Source 0](https://doc.rust-lang.org/book/ch05-01-defining-structs.html).

## Accessing and Updating Struct Fields

You can access the fields of a struct using dot notation (`struct.fieldname`). To update a field, you need to make the struct instance mutable using the `mut` keyword [Source 12](https://www.makeuseof.com/rust-structs-guide-work-with/). Here's an example:

```rust
let mut user1 = User { /* ... */ };
user1.user_name = String::from("Puneet");
```

In this example, the `user_name` field of `user1` is updated to `"Puneet"` [Source 12](https://www.makeuseof.com/rust-structs-guide-work-with/).

## Creating New Instances from Existing Ones

Rust provides a feature called struct update syntax, which allows you to create a new instance of a struct from an existing one. This is done using the `..` syntax [Source 7](https://zerotomastery.io/blog/rust-struct-guide/). Here's an example:

```rust
let user3 = User {
   email: String::from("deepppp@gmail.com"),
   user_name: String::from("shaktiman"),
   ..user1
};
```

In this example, `user3` is a new instance of the `User` struct, with the `email` and `user_name` fields explicitly set, and the rest of the fields copied from `user1` [Source 7](https://zerotomastery.io/blog/rust-struct-guide/).

## Conclusion

Structs in Rust are a powerful tool for organizing related data. They provide a way to name and manipulate groups of data, making your code clearer and easier to understand.



