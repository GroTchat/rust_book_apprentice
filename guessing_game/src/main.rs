use std::io;

use rand::Rng;

use std::cmp::Ordering;

fn main() {
    println!("Guess the number !");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is : {}", secret_number);

    loop {
        println!("Please, enter a number :");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Read user input failed");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };
        println!("Your number : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Higher"),
            Ordering::Greater => println!("Lower"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
