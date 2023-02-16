use std::io::{self, BufRead};

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    println!(
        "{}",
        io::stdin()
            .lock()
            .lines()
            .skip(1)
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|el| el.parse::<i32>().unwrap())
            .min_by_key(|el| (el.abs(), -el))
            .unwrap_or(0)
    );
}
