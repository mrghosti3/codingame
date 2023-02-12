use std::{io, fmt::Display};

macro_rules! read_line {
    ($x:expr) => {
        io::stdin().read_line($x).unwrap()
    };
}

#[derive(Debug, PartialEq, Eq)]
enum BinaryE {
    One,
    Zero,
    Empty,
}

impl BinaryE {
    fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

#[derive(Debug)]
struct BinaryS {
    btype: BinaryE,
    count: u8,
}

impl Display for BinaryS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let count_enc = "0".repeat(self.count as usize);
        match self.btype {
            BinaryE::One => write!(f, "0 {}", count_enc),
            BinaryE::Zero => write!(f, "00 {}", count_enc),
            BinaryE::Empty => write!(f, "")
        }
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    read_line!(&mut input_line);
    let message = input_line.trim_matches('\n').to_string();

    let bin: String = message.bytes().map(|c| format!("{:07b}", c)).collect();

    let mut b_encoded: BinaryS = BinaryS {
        btype: BinaryE::Empty,
        count: 0,
    };

    let mut out_str = String::new();

    for b in bin.bytes() {
        let btype = match b {
            // Case for '1'
            0x31 => BinaryE::One,
            // Case for '0'
            0x30 => BinaryE::Zero,
            _ => unreachable!()
        };

        if b_encoded.btype == btype {
            b_encoded.count += 1;
        } else {
            out_str.push_str(&b_encoded.to_string());
            if !b_encoded.btype.is_empty() {
                out_str.push(' ');
            }
            b_encoded = BinaryS { btype, count: 1 }
        }
    }

    out_str.push_str(&b_encoded.to_string());

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("{}", out_str);
}
