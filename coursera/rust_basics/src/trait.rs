/*
trait -> defines method for types, enable polymorphism, code reuse and abstraction
-> provides default method implementation
-> supports dynamic dispatch via trait obj

trait obj by:
dynamic dispatch: keeps diff types in one collection if they share the same trait
hetrogeneous collections: stores diff types with the same trait in one collection
eg: Vec<Box<dyn Triat>>

dynamic dispatch decides at runtime which version of a polymorphic operation to call



Exploring Techniques for Optimizing Trait Object Usage and Minimizing Runtime Overhead
 Optimizing trait object usage and minimizing runtime overhead in Rust involves a combination of careful design decisions, leveraging compile-time polymorphism, and understanding the trade-offs of dynamic dispatch. Here are some techniques to achieve these goals:

1. Prefer Static Dispatch Where Possible
Static dispatch (also known as monomorphization) generates specialized code for each type at compile-time, eliminating the overhead of dynamic dispatch.

2. Use impl Trait for Function Arguments and Return Types
Using impl Trait for function arguments and return types can provide a balance between type safety and flexibility without incurring the overhead of trait objects.

3. Minimize Trait Object Usage in Performance-Critical Code
When you need dynamic dispatch, limit its usage to parts of the code where it is necessary. For performance-critical sections, try to use static dispatch or inline functions where possible.

4. Use Box<dyn Trait> Only When Necessary
If you must use dynamic dispatch, prefer using Box<dyn Trait> over raw trait objects, as it encapsulates ownership and can help with managing lifetimes and heap allocation.

5. Profile and Benchmark
Use Rust’s profiling tools (like cargo bench and cargo flamegraph) to identify hotspots in your code where trait object usage might be causing performance issues. Optimize based on actual data rather than assumptions.

6. Cache Dynamic Dispatch Results
In some cases, caching the results of operations that involve dynamic dispatch can reduce the frequency of these operations, minimizing overhead.
*/
trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("bark")
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("meow")
    }
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    // using trait obj(&dyn)
    let animals: Vec<&dyn Animal> = vec![&dog, &cat];
    for animal in animals {
        animal.speak();
    }

    // using trait obj(Box) -> provide fix size heap allocated data, so have ownership
    let animals2: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    for animal in animals2 {
        animal.speak();
    }
}
