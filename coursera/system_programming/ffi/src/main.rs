#[link(name = "add")]
unsafe extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    let res = unsafe { add(5, 4) };
    println!("sum is: {}", res);
}
