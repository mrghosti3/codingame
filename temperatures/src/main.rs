use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

const DEFAULT_EMPTY_RES: (i32, i32) = (0, 0);

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let read_line = || {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        input_line
    };

    // the number of temperatures to analyse
    let n = parse_input!(read_line(), usize);
    let binding = read_line();
    let line = binding.split_whitespace().take(n);
    let mut temp: Vec<(i32, i32)> = line
        .clone()
        .zip(line.map(|el| i32::abs(0 - parse_input!(el, i32))))
        .map(|(temp, diff)| (parse_input!(temp, i32), diff))
        .collect();
    temp.sort_by(|a, b| match a.1.cmp(&b.1) {
        std::cmp::Ordering::Equal => b.0.cmp(&a.0),
        e => e,
    });

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("{}", temp.get(0).unwrap_or(&DEFAULT_EMPTY_RES).0);
}
