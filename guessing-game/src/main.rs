use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let rand_upper_bound = 101;
    let expect_message = "Failed to read line!";
    
    println!("Can you guess the correct number?");
    println!("");
    println!("Please input your number: ->");

    let mut your_guess = String::new();
    let secret_guess = rand::thread_rng().gen_range(1..rand_upper_bound);

    println!("Secret number was: {}", secret_guess);

    io::stdin()
        .read_line(&mut your_guess)
        .expect(&expect_message);

    println!("You guessed: {}", your_guess);

    let your_guess: u32 = your_guess.trim().parse().expect("Error parsing value!");

    match your_guess.cmp(&secret_guess) {
        Ordering::Less => println!("Your guess was too small!"),
        Ordering::Greater => println!("Your guess was to big!"),
        Ordering::Equal => println!("Bingo! Your guess was right!")
    }
}
