use std::io;

macro_rules! read_line {
    ($x:expr) => {
        io::stdin().read_line($x).unwrap()
    };
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    read_line!(&mut input_line);
    // the number of points used to draw the surface of Mars.
    let surface_n = parse_input!(input_line, i32);
    for _ in 0..surface_n as usize {
        let mut input_line = String::new();
        read_line!(&mut input_line);
        // let inputs = input_line.split(" ").collect::<Vec<_>>();
        // X coordinate of a surface point. (0 to 6999)
        // let land_x = parse_input!(inputs[0], i32);
        // Y coordinate of a surface point. By linking all the points together in a sequential fashion, you form the surface of Mars.
        // let land_y = parse_input!(inputs[1], i32);
    }

    // game loop
    loop {
        let mut input_line = String::new();
        read_line!(&mut input_line);
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        // let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);
        // the horizontal speed (in m/s), can be negative.
        // let h_speed = parse_input!(inputs[2], i32);
        // the vertical speed (in m/s), can be negative.
        // let v_speed = parse_input!(inputs[3], i32);
        // the quantity of remaining fuel in liters.
        // let fuel = parse_input!(inputs[4], i32);
        // the rotation angle in degrees (-90 to 90).
        // let rotate = parse_input!(inputs[5], i32);
        // the thrust power (0 to 4).
        // let power = parse_input!(inputs[6], i32);

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");


        // 2 integers: rotate power. rotate is the desired rotation angle (should be 0 for level 1), power is the desired thrust power (0 to 4).
        if y > 1000 {
            println!("0 3");
        } else {
            println!("0 4");
        }
    }
}

