use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1, 100);

    println!("The Secret Number is {}", secret_number);

    loop {
        println!("Input a number:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read input");

        let guess: i32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        println!("You guess {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{} is less than the secret number", guess),
            Ordering::Greater => println!("{} is grater than the secret number", guess),
            Ordering::Equal => {
                println!("You won!!!!");
                break;
            }
        }
    }
}
