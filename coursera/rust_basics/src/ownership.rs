/*
Ownership and functions in Rust are tightly linked concepts. Understanding how ownership works when passing values to and returning values from functions is crucial for managing memory safely and effectively.

Key Concepts

Ownership Transfer: When a value is passed to a function, its ownership is transferred to that function unless it's passed by reference.

Borrowing: Passing references allows functions to use values without taking ownership, enabling both mutable and immutable borrowing.

Returning Ownership: Functions can return ownership of values, transferring ownership back to the caller.


Advanced ownership patterns in Rust involve concepts such as borrowing rules, lifetimes, interior mutability, and reference counting. These patterns provide more flexibility in how you manage data and ensure memory safety. 

Some advanced ownership patterns with examples are:

1. Lifetimes

Lifetimes are annotations that describe how long references are valid. They ensure that references do not outlive the data they point to.

2. Interior Mutability

Interior mutability allows you to mutate data even when there are immutable references to it, typically through RefCell or Mutex.

3. Reference Counting

Reference counting with Rc and Arc allows multiple ownership of data. Rc is for single-threaded scenarios, while Arc is for multi-threaded scenarios.

4. Borrowing Across Function Boundaries

Borrowing data across function boundaries while ensuring it doesn't outlive its context can be managed using lifetimes.


*/
