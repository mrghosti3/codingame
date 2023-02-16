use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

macro_rules! half_add {
    ($x:expr, $y:expr) => {
        $x += ($y as f32 / 2.0).ceil() as i32
    };
}

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

    let inputs: Vec<_> = read_line().split(" ").map(|el| el.to_string()).collect();

    // width of the building.
    let w = parse_input!(inputs[0], i32);
    // height of the building.
    let h = parse_input!(inputs[1], i32);
    let mut bnd = Boundries { sx: 0, sy: 0, w, h };

    // maximum number of turns before game over.
    let _n = parse_input!(read_line(), u32);

    let inputs = read_line()
        .split(" ")
        .map(|el| el.to_string())
        .collect::<Vec<_>>();
    let mut x = parse_input!(inputs[0], i32);
    let mut y = parse_input!(inputs[1], i32);

    // game loop
    loop {
        let mut tmp = bnd;

        match read_line().trim().into() {
            BDir::Up => {
                tmp.h = y;
                y -= half_len(y - bnd.sy);
            }
            BDir::Down => {
                tmp.sy = y;
                y += half_len(bnd.h - y);
            }
            BDir::Right => {
                tmp.sx = x;
                x += half_len(bnd.w - x);
            }
            BDir::Left => {
                tmp.w = x;
                x -= half_len(x - bnd.sx);
            }
            BDir::UpRight => {
                tmp.h = y;
                tmp.sx = x;
                y -= half_len(y - bnd.sy);
                x += half_len(bnd.w - x);
            }
            BDir::UpLeft => {
                tmp.h = y;
                tmp.w = x;
                y -= half_len(y - bnd.sy);
                x -= half_len(x - bnd.sx);
            }
            BDir::DownRight => {
                tmp.sy = y;
                tmp.sx = x;
                y += half_len(bnd.h - y);
                x += half_len(bnd.w - x);
            }
            BDir::DownLeft => {
                tmp.sy = y;
                tmp.w = x;
                y += half_len(bnd.h - y);
                x -= half_len(x - bnd.sx);
            },
        }

        y = y.clamp(bnd.sy, bnd.h - 1);
        x = x.clamp(bnd.sx, bnd.w - 1);
        bnd = tmp;
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // the location of the next window Batman should jump to.
        println!("{} {}", x, y);
    }
}

fn half_len(len: i32) -> i32 {
    (len as f32 / 2.0).ceil() as i32
}

#[derive(Debug, Clone, Copy)]
struct Boundries {
    sx: i32,
    sy: i32,
    w: i32,
    h: i32,
}

enum BDir {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl From<&str> for BDir {
    fn from(value: &str) -> Self {
        match value {
            "U" => Self::Up,
            "UR" => Self::UpRight,
            "R" => Self::Right,
            "DR" => Self::DownRight,
            "D" => Self::Down,
            "DL" => Self::DownLeft,
            "L" => Self::Left,
            "UL" => Self::UpLeft,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pos_update() {
        let mut x = 1;
        x -= half_len(x - 0);
        assert_eq!(x, 0);
    }
}
