use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    println!("Please input your guess.");
    
    let mut guess = String::new();

    let guess = guess.trim().parse().expect("Please type a number!");

    io::stdin()
       .read_line(&mut guess)
       .expect("Failed to read line");

    println!("You guessed: {guess}");

    match guess.cmp($secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    // https://doc.rust-kr.org/ch02-00-guessing-game-tutorial.html 추가된 라인은 다음과 같습니다:

}
