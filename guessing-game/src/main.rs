use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number !");
    let rand_num = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        println!("You guessed: {guess}");
        // println!("The AI choose {rand_num}");
        
        let guess: u32 = guess.trim().parse().expect("Please type a number !");
        
        match guess.cmp(&rand_num) {
            Ordering::Less => println!("Too small !"),
            Ordering::Greater => println!("Too big !"),
            Ordering::Equal => println!("Nice ! You win !"),
        }
    }
}
