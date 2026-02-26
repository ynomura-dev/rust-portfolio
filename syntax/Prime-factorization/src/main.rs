use std::io; // To allows input/output functions to be used under the short name “io”.

fn main() {
    println!("Enter a number to factor:");

    let mut factors: Vec<i32> = vec![];

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number: i32 = input.trim().parse().expect("Please type a number!");

    if number <= 1 {
        println!("Please enter an integer greater than 1.");
        return;
    }

    let mut current_number = number;

        for divisor in 2..=current_number/2+1 {
            while current_number % divisor == 0 {
                factors.push(divisor);
                current_number /= divisor;
            }
        }


    println!("The prime factors are: {:?}", factors);
}