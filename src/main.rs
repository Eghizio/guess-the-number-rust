use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

  // println!("Please provide an upper bound of the range from 1:");
  // let max_range = get_number_input() + 1;
  // let number: u64 = rand::thread_rng().gen_range(1..max_range);

  let number: u64 = rand::thread_rng().gen_range(1..100+1);

  loop {
    println!("\nGuess the number:");
    
    let guess = get_number_input();

    match guess.cmp(&number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You guessed it! The number was {}", number);
        println!("\nYou win!");
        break;
      }
    }
  }
}

fn get_number_input() -> u64 {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Please type a number!");

  let input: u64 = match input.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      println!("Please provide a valid number!");
      get_number_input()
    },
  };

  return input;
}