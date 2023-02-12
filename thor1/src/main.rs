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
 * ---
 * Hint: You can use the debug stream to print initialTX and initialTY, if Thor seems not follow your orders.
 **/
fn main() {
    let mut input_line = String::new();
    read_line!(&mut input_line);

    let inputs = input_line.split(" ").collect::<Vec<_>>();
    // the X position of the light of power
    let light_x = parse_input!(inputs[0], i32);
    // the Y position of the light of power
    let light_y = parse_input!(inputs[1], i32);
    // Thor's starting X position
    let initial_tx = parse_input!(inputs[2], i32);
    // Thor's starting Y position
    let initial_ty = parse_input!(inputs[3], i32);

    let mut tx = initial_tx;
    let mut ty = initial_ty;

    // game loop
    loop {
        let mut input_line = String::new();
        read_line!(&mut input_line);
        let _remaining_turns = parse_input!(input_line, i32); // The remaining amount of turns Thor can move. Do not remove this line.

        if ty < light_y {
            print!("S");
            ty += 1;
        } else if light_y < ty {
            print!("N");
            ty -= 1;
        }

        if tx < light_x {
            print!("E");
            tx += 1;
        } else if light_x < tx {
            print!("W");
            tx -= 1;
        }
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // A single line providing the move to be made: N NE E SE S SW W or NW
        println!();
        // println!("SE");
    }
}
