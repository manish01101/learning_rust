use std::io;

fn main() {
    /* VARIABLE AND MUTABILITY */
    let var_x = 5;
    let var_x = var_x + 1; // shadowing
    let mut var_y = 5;
    var_y = 10;
    const PI: f32 = 3.14; // uppercase convention
    let default_floating = 3.0; // default f64
    {
        let var_x = var_x * 2; // shadowing
        println!("val of var_x: {var_x}");
    }
    println!("val of var_x: {var_x}");
    println!("val of var_y: {var_y}");
    println!("val of pi: {PI}\n");

    /* SCALAR DATA TYPE: INT, FLOATING, BOOLEAN, CHAR */
    let int = 5 + 10;
    let floating = 55.5 - 4.0;
    let boolean = true;
    let heart_eyed_cat = '😻'; // rust char is 4 bytes, it supports unicode scalar value

    println!("{int}");
    println!("{floating}");
    println!("{boolean}");
    println!("{heart_eyed_cat}\n");

    /* COMPOUND DATA TYPE: TUPLE & ARRAY */
    // tuple
    let tup: (i32, f64, u8) = (500, 3.5, 1);
    let (x, y, z) = tup; // destructuring
    println!("y is {y}, x is {x}, z is {z}");
    let first = tup.0; // access using index
    let last = tup.2;
    println!("first is {first}, last is {last}\n");

    // array
    let arr: [i32; 5] = [1, 3, 5, 2, 4];
    let ar = [3; 4]; // [3, 3, 3, 3]
    let third = arr[2];
    let a = ar[3];
    println!("arr ele is {third}, ar ele is {a}");

    let a = [1, 2, 3, 4, 5];
    println!("enter arr index");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("fail to read");
    let index: usize = index.trim().parse().expect("index entered not a number");

    let ele = a[index];
    println!("the val at index {index} is {ele}");

    let condition = true;
    let n = if condition { 5 } else { 6 };
    println!("n is {n}\n");

    /* FUNCTIONS AND PARAMETER */
    fn print_fn(val: i32) {
        println!("val is: {val}");
    }

    /* STATEMENTS AND EXPRESSION */
    /*
    statement do not return values, expression return values,
    Expressions do not include ending semicolons. If
    you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
    */

    /* IF EXPRESSION */
    let num = 5;
    if num < 5 {
        // condition must be bool type only any data val not work as it do in c++
        println!("condition is passed");
    } else if num == 5 {
        println!("condition is accurate");
    } else {
        println!("not passed");
    }

    /*  LOOP -> returning values from loops */
    let mut ans = 0;
    let res = loop {
        ans += 1;
        if ans == 10 {
            break ans * 2; // val after break expression return it outside of loop
        }
    };
    println!("ans is {res}\n");

    /* LOOP LABELS TO DISAMBIGUATE B/W MULTIPLE LOOPS */
    let mut cnt = 0;
    // loop label must begin with single quote
    'counting_up: loop {
        println!("count = {cnt}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if cnt == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        cnt += 1;
    }
    println!("end count = {cnt}");

    /* CONDITIONAL LOOP WITH WHILE */
    let mut n = 3;
    while n != 0 {
        println!("{n}");
        n -= 1;
    }

    /* FOR LOOP */
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
    for number in (1..4).rev() {
        // rev is used to reverse the range
        println!("{number}!");
    }
}
