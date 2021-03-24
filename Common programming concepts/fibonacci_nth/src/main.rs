use std::io::{self};

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("failed to read a line");
    let n: i32 = line.trim().parse().expect("Failed to parse number");
    println!("{:.0}", (((1.0 + 5.0_f64.sqrt()) / 2.0).powi(n) - ((1.0 - 5.0_f64.sqrt()) / 2.0).powi(n)) / 5.0_f64.sqrt());
}
