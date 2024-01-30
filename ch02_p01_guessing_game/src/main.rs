use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is: {secret_number}");

    loop {

        println!("Please input your guess.");
        let mut guess = String::new();
        
        // referencias son inmutables por default
        // en .read_line(&mut guess) aclara el mut ya que modifica guess.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        // puedo usar la misma variable para cambiar de un tipo a otro
        // y no tener que crear una variable nueva
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Try again! Input a valid number.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
