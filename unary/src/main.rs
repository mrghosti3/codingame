macro_rules! read_line {
    ($x:expr) => {
        std::io::stdin().read_line($x).unwrap()
    };
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    read_line!(&mut input_line);
    let message = input_line.trim_matches('\n').to_string();

    let binding = message.bytes().map(|c| format!("{:07b}", c)).collect::<String>();
    let bin_bytes = binding.as_bytes();

    let mut last_char = bin_bytes[0];

    match bin_bytes[0] {
        0x30 => { print!("00 "); }
        0x31 => { print!("0 "); }
        _ => unreachable!(),
    }

    for i in 0..bin_bytes.len() {
        if bin_bytes[i] != last_char {
            last_char = bin_bytes[i];
            match bin_bytes[i] {
                0x30 => { print!(" 00 "); }
                0x31 => { print!(" 0 "); }
                _ => unreachable!(),
            }
        }

        print!("0");
    }


    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!();
}
