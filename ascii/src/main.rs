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

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    read_line!(&mut input_line);
    let len = parse_input!(input_line, usize);
    let mut input_line = String::new();
    read_line!(&mut input_line);
    let height = parse_input!(input_line, usize);
    let mut input_line = String::new();
    read_line!(&mut input_line);
    let text = input_line.trim_matches('\n').to_string();

    let mut res: Vec<String> = Vec::with_capacity(height);
    for i in 0..height {
        res.push(String::with_capacity(text.len() * len));

        // reads the line of ASCII chars
        let mut input_line = String::new();
        read_line!(&mut input_line);
        let row = input_line.trim_matches('\n').to_string();

        // constructs each row of *res* by parsing given input str and matching each ASCII symbol
        // in *row*.
        text.chars()
            .map(|c| {
                let c = c.to_ascii_uppercase();
                if c.is_ascii_uppercase() {
                    return (c as u8 - 65) as usize * len;
                }

                26 * len
            })
            .for_each(|off| {
                res[i].push_str(row.get(off..off + len).unwrap());
            });
    }

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    for ele in res {
        println!("{}", ele);
    }
}
