use std::io;

fn main() {
    let n = read_line()[0];
    let digits: Vec<u32> = n
        .to_string()
        .chars()
        .map(|c: char| c.to_digit(10).unwrap())
        .collect();

    let mut count1 = 0;
    let mut count2 = 0;
    let mut count3 = 0;

    for digit in digits {
        match digit {
            1 => count1 += 1,
            2 => count2 += 1,
            3 => count3 += 1,
            _ => (),
        }
    }

    if count1 == 1 && count2 == 2 && count3 == 3 {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn read_line() -> Vec<i64> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
