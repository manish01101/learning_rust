fn main() {
    /*
    In Rust, vectors (Vec<T>) are dynamic arrays that can grow or shrink at runtime. While Rust has a single main vector type, there are different ways to use and manipulate vectors based on ownership, allocation, and mutability. Here are the different types and variations of vectors in Rust: */

    // 1. Standard Vec<T> (Heap-Allocated, Growable Vector)
    // The most common and widely used vector type.
    // It is dynamically allocated on the heap and can grow or shrink.
    // Example:

    let mut v: Vec<i32> = Vec::new(); // Empty vector
    v.push(1);
    v.push(2);
    println!("{:?}", v); // Output: [1, 2]

    // Shorthand using vec! macro:
    let v = vec![1, 2, 3];

    // 2. &[T] (Slice of a Vector)
    // A reference to a contiguous section of a vector.
    // Does not own the data, only borrows it.
    // Example:

    let v = vec![10, 20, 30, 40];
    let slice: &[i32] = &v[1..3];
    println!("{:?}", slice); // Output: [20, 30]

    // 3. VecDeque<T> (Double-ended Queue)
    // A deque (double-ended queue) that allows efficient pushing and popping from both ends.
    // Useful when you need queue-like behavior.
    // Example:

    use std::collections::VecDeque;

    let mut dq: VecDeque<i32> = VecDeque::new();
    dq.push_back(1); // Push at the back
    dq.push_front(2); // Push at the front
    println!("{:?}", dq); // Output: [2, 1]

    // 4. Box<[T]> (Heap-Allocated Fixed-Size Array)
    // Converts a Vec<T> into a heap-allocated array.
    // Unlike Vec<T>, it cannot grow or shrink.
    // Example:

    let v = vec![1, 2, 3];
    let boxed_slice: Box<[i32]> = v.into_boxed_slice();
    println!("{:?}", boxed_slice);

    // 5. Rc<Vec<T>> / Arc<Vec<T>> (Reference-Counted Vectors)
    // Used for shared ownership of a vector.
    // Rc<Vec<T>> (single-threaded) and Arc<Vec<T>> (multi-threaded).

    // Example (Rc for single-threaded):
    use std::rc::Rc;

    let shared_vec = Rc::new(vec![1, 2, 3]);
    let cloned_vec = Rc::clone(&shared_vec);
    println!("{:?}", cloned_vec); // Output: [1, 2, 3]

    // Example (Arc for multi-threaded):
    use std::sync::Arc;

    let shared_vec = Arc::new(vec![1, 2, 3]);
    let cloned_vec = Arc::clone(&shared_vec);
    println!("{:?}", cloned_vec);

    // 6. Cow<[T]> (Copy-on-Write Vector)
    // Used when you want to avoid unnecessary cloning.
    // Efficient for read-heavy and rarely modified data.

    // Example:
    use std::borrow::Cow;

    let numbers: Cow<[i32]> = Cow::from(vec![1, 2, 3]); // Clones only if modified
    println!("{:?}", numbers);

    /*
    Summary Table
    Type	Description
    Vec<T>	Standard growable vector (heap-allocated).
    &[T]	Slice of a vector (borrowed).
    VecDeque<T>	Double-ended queue (push/pop at both ends).
    Box<[T]>	Heap-allocated fixed-size array.
    Rc<Vec<T>>	Reference-counted vector (single-threaded).
    Arc<Vec<T>>	Atomic reference-counted vector (multi-threaded).
    Cow<[T]>	Copy-on-Write (efficient for rarely modified data
    */
}
