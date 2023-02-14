use std::io;

macro_rules! read_line {
    ($x:expr) => {
        io::stdin().read_line($x).unwrap()
    };
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

const N: usize = 100000;

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    read_line!(&mut input_line);
    let n = parse_input!(input_line, i32);

    let mut strengths = [0i32; N];
    for i in 0..n as usize {
        let mut input_line = String::new();
        read_line!(&mut input_line);
        strengths[i] = parse_input!(input_line, i32);
    }

    strengths.sort_by(|a, b| b.cmp(a));

    let mut min_diff = strengths[0];
    let mut prev = strengths[0];

    strengths[1..n as usize].iter().for_each(|el| {
        let tmp = prev - el;
        if min_diff > tmp {
            min_diff = tmp;
        }

        prev = *el;
    });

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("{}", min_diff);
}
