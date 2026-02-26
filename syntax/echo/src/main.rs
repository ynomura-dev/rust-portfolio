use std::io; // Standard library for input/output

fn main() {
    println!("Please enter some number and press Enter (press Ctrl+C to exit)");
    loop {
        let mut input = String::new(); //Standard input is read as a string.

        // 2. Wait for input from the keyboard (standard input)
        io::stdin()
            .read_line(&mut input) // Write input into the input variable
            .expect("Failed to read line"); // Stop if an error occurs

        // 3. Display the input (Echo)
        // Since input includes the newline character, print! is used here.
        print!("You entered: {}", input);
    }
}
