use std::{io, ops::{Div, Mul}};

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
    let lon = input_line.trim().to_string();

    let mut input_line = String::new();
    read_line!(&mut input_line);
    let lat = input_line.trim().to_string();

    let mut input_line = String::new();
    read_line!(&mut input_line);
    let n = parse_input!(input_line, usize);

    let point_a: (&str, &str) = (&lon, &lat);
    let mut closest = Defib::default();
    for _ in 0..n as usize {
        let mut input_line = String::new();
        read_line!(&mut input_line);
        let mut defib = Defib::parse_str(input_line.trim_matches('\n'));

        if closest.dist(point_a) > defib.dist(point_a) {
            closest = defib;
        }
    }

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("{}", closest.name);
}

#[derive(Debug)]
struct Defib {
    // id: u16,
    name: String,
    // addr: String,
    // phone: Option<String>,
    lon: f64,
    lat: f64,
    _dist: Option<f64>,
}

impl Default for Defib {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            lon: 0.0,
            lat: 0.0,
            _dist: Some(f64::MAX),
        }
    }
}

impl Defib {
    fn parse_str(input: &str) -> Self {
        let split_in: Vec<&str> = input.split(";").collect();

        // let id: u16 = split_in[0].parse().unwrap();
        let name = split_in[1].to_string();
        // let addr = split_in[2].to_string();
        // let phone = split_in[3].is_empty().then_some(split_in[3].to_string());
        let lon: f64 = split_in[4].replace(',', ".").parse().unwrap();
        let lat: f64 = split_in[5].replace(',', ".").parse().unwrap();

        // Defib { id, name, addr, phone, long, lat }
        Defib {
            name,
            lon: lon.to_radians(),
            lat: lat.to_radians(),
            _dist: None,
        }
    }

    fn dist(&mut self, point_a: (&str, &str)) -> f64 {
        if let Some(dist) = self._dist {
            return dist;
        }

        let (lon_a, lat_a) = (
            point_a.0.replace(',', ".").parse::<f64>().unwrap().to_radians(),
            point_a.1.replace(',', ".").parse::<f64>().unwrap().to_radians(),
        );

        let x = (self.lon - lon_a) * f64::cos((lat_a + self.lat).div(2.0));
        let y = self.lat - lat_a;
        self._dist = Some(f64::sqrt(x.powi(2) + y.powi(2)).mul(6371.0));
        self._dist.unwrap()
    }
}
