use std::io; //Use standard library for input and output
use rand::Rng; //Random number generation
use std::cmp::Ordering; //Comparison trait

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); //create empty string

        io::stdin() //get standard input hundle
            .read_line(&mut guess) //subject omission (Method Chain). "&" denotes a reference
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        //.trim() removes whitespace/newline characters like "\n"," ".
        //.parse() converts string to number

        match guess.cmp(&secret_number) {//.cmp() returns "std::cmp::Ordering::Less, Greater, or Equal".
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("You guessed: {}", guess);
    }
}