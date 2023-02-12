use std::{io, collections::HashMap};

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

    // Number of elements which make up the association table.
    let n = parse_input!(input_line, i32);

    let mut input_line = String::new();
    read_line!(&mut input_line);
    // Number Q of file names to be analyzed.
    let q = parse_input!(input_line, i32);

    let mut mime_table = HashMap::new();

    for _ in 0..n as usize {
        let mut input_line = String::new();
        read_line!(&mut input_line);
        let inputs = input_line.split(" ").collect::<Vec<_>>();

        // file extension
        let ext = inputs[0].trim().to_string().to_lowercase();
        // MIME type.
        let mt = inputs[1].trim().to_string();

        mime_table.insert(ext, mt);
    }

    for _ in 0..q as usize {
        let mut input_line = String::new();
        read_line!(&mut input_line);
        let fname = match input_line.rsplit_once('.') {
            Some((_, mtype)) => mtype.trim().to_lowercase(),
            None => "UNKNOWN".to_string()
        };

        println!("{}", mime_table.get(&fname).unwrap_or(&"UNKNOWN".to_string()));
    }

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    // For each of the Q filenames, display on a line the corresponding MIME type. If there is no corresponding type, then display UNKNOWN.
    // println!("UNKNOWN");
}
