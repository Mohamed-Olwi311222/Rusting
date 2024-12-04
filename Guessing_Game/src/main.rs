use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");
    loop {
        let mut guess = String::new();
        println!("Please input your guess.");
        //get input from user
        io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

        //parse the input from string to int and check if it is a number
        let guess : u32 = match guess
        .trim()
        .parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You have guessed {}", guess);

        //compare the number inputed 
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal=> {
                println!("You guessed right!");
                break;
            }
            Ordering::Greater=> println!("Too Big!")
         }
    }
   print!("The secret number is {secret_number}");

}
