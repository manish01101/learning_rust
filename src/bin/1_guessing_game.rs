use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_num = rand::thread_rng().gen_range(1..=100);
    // println!("the secret num is: {}", secret_num);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables
        // let guess: u32 = guess.trim().parse().expect("please type a number!");
        // handling invalid i/p
        //“parse returns a Result type and Result is an enum that has the variants Ok and Err”
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
             Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
