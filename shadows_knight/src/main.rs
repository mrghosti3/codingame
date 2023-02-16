use std::io;

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
    let inputs: Vec<_> = read_line().split(" ").map(|el| el.to_string()).collect();

    // width of the building.
    let w = parse_input!(inputs[0], i32);
    // height of the building.
    let h = parse_input!(inputs[1], i32);

    // maximum number of turns before game over.
    let _n = read_line();

    let inputs = read_line()
        .split(" ")
        .map(|el| el.to_string())
        .collect::<Vec<_>>();
    let mut x = parse_input!(inputs[0], i32);
    let mut y = parse_input!(inputs[1], i32);

    let mut lw = 0;
    let mut uw = w;
    let mut lh = 0;
    let mut uh = h;

    // game loop
    loop {
        let dir = read_line();

        if dir.starts_with('U') {
            uh = y;
        } else if dir.starts_with('D') {
            lh = y;
        }

        if dir.ends_with('L') {
            uw = x;
        } else if dir.ends_with('R') {
            lw = x;
        }


        y = (lh + uh) / 2;
        x = (lw + uw) / 2;
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // the location of the next window Batman should jump to.
        println!("{} {}", x, y);
    }
}

fn read_line() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line.trim().to_string()
}
