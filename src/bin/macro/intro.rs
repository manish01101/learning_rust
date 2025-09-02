// display trait -> there is not inbuilt macro for display trait
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(name is {}, age is {})", self.name, self.age)
    }
}
// debug trait -> it has inbuilt trait
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(name is {}, age is {})", self.name, self.age)
    }
}
//        or
// #[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    let u = User {
        name: "manish".to_string(),
        age: 22,
    };

    println!("{}", u); // use display trait
    println!("{:?}", u); // use debug trait
}
