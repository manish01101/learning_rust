/*
A trait is like an interface that data types can implement. When a type implements a trait it can be treated abstractly as that trait using generics or trait objects.
Traits = similar to abstract classes or interfaces in C++.
Traits can be made up of three varieties of associated items:
>> functions and methods
>> types
>> constants

impl Trait in Rust ≈ templates in C++ (compile-time polymorphism).
&dyn Trait in Rust ≈ virtual base-class pointer/reference in C++ (runtime polymorphism).
| Aspect          | `&impl Shape` (Static)                          | `&dyn Shape` (Dynamic)               |
| --------------- | ----------------------------------------------- | ------------------------------------ |
| Dispatch type   | Compile-time (static)                           | Runtime (dynamic) via vtable         |
| Function copies | One per type (monomorphization)                 | One single function                  |
| Speed           | Faster (inlining possible)                      | Slightly slower (vtable indirection) |
| Flexibility     | Can only accept **one concrete type** at a time | Can accept **any Shape** at runtime  |
| Collections     | `Vec<Rectangle>` or `Vec<Square>` only          | `Vec<&dyn Shape>` can mix types      |
| C++ analogy     | Templates                                       | Virtual functions                    |

 */

use std::time::Instant;
trait Shape {
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
}

struct Rectangle {
    length: u32,
    breadth: u32,
}
struct Square {
    side: u32,
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.length * self.breadth
    }
    fn perimeter(&self) -> u32 {
        2 * (self.length + self.breadth)
    }
}
impl Shape for Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
    fn perimeter(&self) -> u32 {
        4 * self.side
    }
}
// This uses impl Shape, which is static dispatch (the compiler knows the exact type at compile time and generates optimized code).
fn area_and_peri(s: &impl Shape) -> u32 {
    // println!("area is {} and perimeter is {}", s.area(), s.perimeter());
    s.area() + s.perimeter()
}

// Trait Objects in Rust (Runtime Polymorphism)
// &dyn Shape is a trait object, like a base-class pointer in C++.
// Rust will use dynamic dispatch (vtable lookup) just like C++’s virtual functions.
// If you want runtime polymorphism, you can use a trait object (&dyn Shape or Box<dyn Shape>):
fn area_and_peri_dynamic(s: &dyn Shape) -> u32 {
    // println!("area is {} and perimeter is {}", s.area(), s.perimeter());
    s.area() + s.perimeter()
}

fn main() {
    let r = Rectangle {
        length: 3,
        breadth: 4,
    };
    let sq = Square { side: 300 };

    let iterations = 100_000_000;

    // static dispatch
    let start = Instant::now();
    let mut sum1: u128 = 0;
    for _ in 0..iterations {
        sum1 += area_and_peri(&r) as u128;
        sum1 += area_and_peri(&sq) as u128;
    }
    let dur_static = start.elapsed();

    // dynamic dispatch
    let start = Instant::now();
    let mut sum2: u128 = 0;
    for _ in 0..iterations {
        sum2 += area_and_peri_dynamic(&r) as u128;
        sum2 += area_and_peri_dynamic(&sq) as u128;
    }
    let dur_dynamic = start.elapsed();

    println!("Static dispatch time   : {:?}", dur_static);
    println!("Dynamic dispatch time  : {:?}", dur_dynamic);
    println!("Ignore: {} {}", sum1, sum2);
}

/* output
Static dispatch time   : 3.391858375s
Dynamic dispatch time  : 3.410131417s
Ignore: 9122600000000 9122600000000
*/
