use std::{fmt::Display, io};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Don't let the machines win. You are humanity's last hope...
 **/
fn main() {
    // the number of cells on the X axis
    let width = parse_input!(read_line(), u8);

    // the number of cells on the Y axis
    let height = parse_input!(read_line(), u8);
    let mut matrix: Vec<Vec<Option<Node>>> = Vec::with_capacity(height as usize);

    for y in 0..height {
        // width characters, each either 0 or .
        let line: Vec<_> = read_line().chars().collect();
        matrix.push(Vec::with_capacity(width as usize));

        for x in 0..width {
            if line[x as usize] == '.' {
                matrix[y as usize].push(None);
                continue;
            }

            matrix[y as usize].push(Some(Node::new(x, y)));
        }
    }

    // Write an action using println!("message...");
    // To debug: eprintln!("Debug message...");

    // Three coordinates: a node, its right neighbor, its bottom neighbor
    matrix.iter().flatten().for_each(|el| {
        if let Some(node) = el {
            println!("{} {}", node, node.neighbor(&matrix));
        }
    });
}

fn read_line() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line.trim().to_string()
}

#[derive(Debug, Clone, Copy)]
struct Node {
    x: u8,
    y: u8,
}

impl Node {
    fn new(x: u8, y: u8) -> Self {
        Node { x, y }
    }

    fn neighbor(&self, matrix: &[Vec<Option<Node>>]) -> String {
        let mut res = String::with_capacity(7);

        for el in matrix[self.y as usize].iter().skip(self.x as usize + 1) {
            if let Some(node) = el {
                res.push_str(&format!("{} {} ", node.x, node.y));
                break;
            }
        }

        if res.is_empty() {
            res.push_str("-1 -1 ");
        }

        let mut bottom_n = None;
        for row in matrix.iter().skip(self.y as usize + 1) {
            // x cache
            let xc = self.x as usize;
            if row[xc].is_some() {
                bottom_n = row[xc];
                break;
            }
        }

        match bottom_n {
            Some(rn) => res.push_str(&format!("{} {}", rn.x, rn.y)),
            None => res.push_str("-1 -1")
        }

        res
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}
