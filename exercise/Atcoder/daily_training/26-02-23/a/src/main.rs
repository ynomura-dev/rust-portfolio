use std::io;

fn main() {
    let v = read_line();
    let _n = v[0];
    let _m = v[1];
    let a = read_line();
    let b = read_line();
    let mut ans = 0;

    for i in b {
        ans += a[(i - 1) as usize];
    }
    println!("{}", ans);
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
