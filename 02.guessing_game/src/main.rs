extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::prelude::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut tries = 0;
    
    loop {
          println!("Please input you guess (1-100).");
          let mut guess = String::new();
          io::stdin().read_line(&mut guess)
              .expect("Failed to read line");           // borrow mut guess 
          let guess: u32 = match guess.trim().parse() { // extract num from io::Result<usize>
              Ok(num) => num, 
              Err(_) => continue,
          };
          tries += 1;  // mutable
          println!("You guessed: {}", guess);
          match guess.cmp(&secret_number) {
              Ordering::Less => println!("Too small! Try #{}", tries),
              Ordering::Greater => println!("Too big! Try #{}", tries),
              Ordering::Equal => {
                  println!("You win! On #{} tries", tries);
                  break;
              }
          }
      }
    }

