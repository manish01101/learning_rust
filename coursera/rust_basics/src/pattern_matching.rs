fn main() {
    let num = vec![1, 2, 3, 4];
    match &num[..] {
        [first, rest @ ..] => println!("first: {}, rest: {:?}", first, rest),
        _ => println!("empty arr"),
    }
}
