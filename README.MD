# RUST tutorial

## to run .rs file we can use two method
1. ``rustc "rustfile" ``
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


# Rust Collections: Vectors, Strings, and HashMaps

This repository contains a Rust program that demonstrates the usage of vectors, strings, and hashmaps. These are fundamental data structures in Rust that allow us to store multiple values.

## Vectors

Vectors in Rust are similar to arrays, but they can grow and shrink dynamically. They are stored on the heap, which allows them to resize as needed.

Here's an example of how to initialize and manipulate a vector:

```rust
let mut v: Vec<i32> = Vec::new(); // Initialize an empty vector

// Add items to the vector using push
v.push(4);
v.push(5);
v.push(6);
v.push(7);

// Access values of the vector using index
let mut third_value = &v[2];
println!("{}", third_value);

// Use the get method to safely access values by index
let fourth = v.get(3);
match fourth {
   Some(fourth) => println!("here is your data at index 4 {}", fourth),
   None => println!("No data found"),
}

// Loop through the vector
for i in &mut v {
   *i += 80;
}
println!("{:?}", v);
```

## Strings

Strings in Rust are UTF-8 encoded and are stored on the heap. They can be initialized using `String::new()`, `data.to_string()`, or `String::from("hello")`.

Here's an example of how to update and manipulate strings:

```rust
let mut s = String::from("foo");
s.push_str("deppak"); // Append a string slice
s.push('!'); // Append a single character
println!("{}", s);

let s2 = String::from("hello");
let s3 = String::from("Deepak");
let s4 = format!("{}{}", s2, s3); // Concatenate strings
println!("{}", s4);
```

Note: Indexing into a string is not allowed because a character in a string can take up more than one byte.

## HashMaps

HashMaps in Rust are similar to dictionaries in Python or objects in JavaScript. They store key-value pairs and allow for efficient lookup, insertion, and deletion of elements.

Here's an example of how to initialize and manipulate a hashmap:

```rust
use std::collections::HashMap;

let key1 = String::from("Blue");
let key2 = String::from("Reds");

let mut scores = HashMap::new();

// Insert key-value pairs into the hashmap
scores.insert(&key1, 10);
scores.insert(&key2, 20);

// Access values by key
let value = scores.get(&key1);
match value {
   Some(value) => println!("found {}", value),
   None => println!("nothing found"),
}

// Insert a key-value pair only if the key isn't already present
let new_key = String::from("Black");
scores.entry(&new_key).or_insert(69);

// Print key-value pairs
for (key, value) in &scores {
   println!("key : {}, value: {}", key, value)
}
```

In this example, `scores.entry(&new_key).or_insert(69);` will insert the key "Black" with the value `69` into the hashmap only if the key "Black" does not already exist in the hashmap.


# Error Handling in Rust

Rust provides a few key concepts and tools for error handling. The primary ones are the `Result<T, E>` enum and the `Option<T>` enum.

## The `Result<T, E>` Enum

The `Result<T, E>` enum is used for operations that can fail. It has two variants: `Ok(T)`, which represents success and contains a value of type `T`, and `Err(E)`, which represents failure and contains an error of type `E` [Source 1](https://doc.rust-lang.org/book/ch09-00-error-handling.html).

Here's an example of a function that returns a `Result`:

```rust
use std::fs::File;

fn open_file(filename: &str) -> std::io::Result<File> {
   File::open(filename)
}
```

In this example, `open_file` tries to open a file and returns a `Result`. If it succeeds, it returns `Ok(File)`. If it fails, it returns `Err(io::Error)`.

## The `Option<T>` Enum

The `Option<T>` enum is used for operations that might not return a value. It has two variants: `Some(T)`, which represents a value of type `T`, and `None`, which represents the absence of a value [Source 1](https://doc.rust-lang.org/book/ch09-00-error-handling.html).

Here's an example of a function that returns an `Option`:

```rust
fn find_first_even_number(vec: Vec<i32>) -> Option<i32> {
   vec.into_iter().find(|&x| x % 2 == 0)
}
```

In this example, `find_first_even_number` tries to find the first even number in a vector and returns an `Option`. If it finds an even number, it returns `Some(i32)`. If it doesn't find an even number, it returns `None`.

## Error Propagation

Rust uses the `?` operator to propagate errors. When you see `?` after an expression, it means "if this expression is `Err`, return early from the function and give the error to the caller". If the expression is `Ok`, it unwraps the value inside and continues executing the rest of the function [Source 0](https://dev.to/nathan20/how-to-handle-errors-in-rust-a-comprehensive-guide-1cco).

Here's an example of error propagation:

```rust
use std::fs::File;

fn read_file_contents(filename: &str) -> std::io::Result<String> {
   let mut file = File::open(filename)?;
   let mut contents = String::new();
   file.read_to_string(&mut contents)?;
   Ok(contents)
}
```

In this example, if `File::open` or `file.read_to_string` returns an `Err`, `read_file_contents` will return early with that error. If they both succeed, it returns `Ok(contents)`.




## Best Practices

When handling errors in Rust, it's important to:

- Use the `Result<T, E>` enum for operations that can fail, and the `Option<T>` enum for operations that might not return a value.
- Use the `?` operator to propagate errors up the call stack.
- Define your own error types to provide more context about what went wrong.



