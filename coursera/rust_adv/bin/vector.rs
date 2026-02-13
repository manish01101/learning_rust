fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let split: Vec<&[i32]> = v.split(|&x| x == 3).collect();
    println!("{:?}", split);
    let split2 = v.split_at(2);
    println!("{:?}", split2);

    // immutable iteration
    for e in split.iter() {
        println! {"{:?}", e};
    }
    // mutable iteration
    let split3 = &mut v[1..3];
    for e in split3.iter_mut() {
        println! {"{:?}", e};
    }
    // Accessing Elements
    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);
    // Iterating Over a Vector

    for i in &v {
        println!("{}", i);
    }
    for i in v.iter() {
        println!("{}", i);
    }
    // Removing Elements
    let last = v.pop();
    let second = v.remove(1);
    v.retain(|&x| x % 2 == 0);
}
