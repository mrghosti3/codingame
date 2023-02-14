use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();

    let mut lines = stdin.lock().lines().map(|x| x.unwrap());

    let _n: usize = lines.next().unwrap().parse().unwrap();

    let mut strengths: Vec<i32> = lines.map(|x| x.parse().unwrap()).collect();

    strengths.sort();

    let mindiff: i32 = strengths
        .iter()
        .zip(strengths.iter().skip(1))
        .map(|(x, y)| (x - y).abs())
        .min()
        .unwrap();

    println!("{}", mindiff);
}
