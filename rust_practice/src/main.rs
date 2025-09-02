fn main() {
    println!("Hello, world!");
    // variable
    // let mut aa = "manish";
    // aa = "name";
    // let mut b: i128 = 44444444444443544444444444444444444444;
    // b = 0;


    // fn
    // add_fn(44);
    // println!("a+b: {}", pass_fn(5, 5))


    // if else
    // let a: i128 = 555;
    // if a>500 {
    //     println!("greater")
    // } else {
    //     println!("smaller")
    // }


    // let mut a = 0;
    // loop {
    //     if a == 5 {
    //         break;
    //     } 
    //     println!("{a}");
    //     a+=1;
    // }
    // while a != 5 {
    //     println!("while :{}", a);
    //     a+=1
    // }


    // match (similar to switch)
    // let x = 3;
    // match x {
    //     1 => println!("its 1"),
    //     2 => println!("its 2"),
    //     _ => println!("its something else"),
    // }


    


    // structure
    struct GroceryItem {
        stock: i32,
        price: f64,
    }
    let cereal = GroceryItem {
        stock: 3,
        price: 2.433,
    };
    println!("stock: {}, price: {}", cereal.stock, cereal.price)
}

// fn add_fn(x : i32) {
//     println!("val of x: {:?}", x); //:?->debugging
//     println!("val of x: {x:?}"); //:?->debugging
// }
// fn pass_fn(y: i64, z: i64) -> i64 {
//     y + z
// }
