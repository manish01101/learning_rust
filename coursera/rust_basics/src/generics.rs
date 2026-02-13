/*
| Aspect       | Trait Objects                                   | Generics                                        |
| ------------ | ----------------------------------------------- | ----------------------------------------------- |
| Dispatch     | Dynamic dispatch                                | Static dispatch                                 |
| Performance  | Runtime cost due to dynamic dispatch            | Compile-time cost, but faster at runtime        |
| Type Erasure | Yes                                             | No                                              |
| Flexibility  | More flexible, allows heterogeneous collections | Less flexible, requires homogeneous collections |
| Syntax       | `Box<dyn Trait>`                                | `T: Trait`                                      |
| Memory Usage | Higher, due to additional indirection           | Lower, as no indirection                        |
| Type Safety  | Runtime check                                   | Compile-time check                              |



Generic Traits

Generic traits allow you to define traits that can operate over a variety of types, enabling polymorphism and code reuse. By defining a trait with generics, you can specify methods that can accept parameters of different types or return different types, as long as those types meet certain constraints.

Generic Lifetimes

Lifetimes in Rust are used to specify how long references should be valid. Generic lifetimes allow you to define functions and structs that can operate on references with lifetimes that are determined at runtime. They are also used to ensure that references do not outlive the data they point to, preventing dangling references and memory safety issues.

By combining the features of both, Rust allows you to write flexible, reusable, and safe code, making it easier to manage complex data and behaviors.
*/

fn generic_function<T>(params: T) -> T {
    params
}
enum GenericEnum<T> {
    Some(T),
    None,
}
struct GenericStruct<T> {
    field: T,
}

// generic method for generic struct
impl<T> GenericStruct<T> {
    fn get_field(&self) -> &T {
        &self.field
    }
}

fn main() {
    let gen_struct = GenericStruct { field: "manish" };
}
