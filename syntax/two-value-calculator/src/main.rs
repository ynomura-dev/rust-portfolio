use std::io;

fn main() {
    println!("Enter the first number:");
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");
    let number1: i32 = input1.trim().parse().expect("Please type a number!");

    println!("Enter the second number:");
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");
    let number2: i32 = input2.trim().parse().expect("Please type a number!");

    println!("Choose an operation (+, -, *, /):");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");
    let operation = operation.trim();

    let result = match operation {
        "+" => number1 + number2,
        "-" => number1 - number2,
        "*" => number1 * number2,
        "/" => {
            if number2 == 0 {
                println!("Error: Division by zero is not allowed.");
                return;
            } else {
                number1 / number2
            }
        }
        _ => {
            println!("Invalid operation selected.");
            return;
        }
    };
    println!("The result of {} {} {} is: {}", number1, operation, number2, result);
}