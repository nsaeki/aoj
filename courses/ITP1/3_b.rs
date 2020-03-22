#[allow(unused_imports)]
use std::cmp::{max, min};
use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let out = &mut BufWriter::new(stdout());

    for i in 0..10000 {
        let mut input = String::new();
        stdin().read_line(&mut input).ok();
        match input.trim().parse::<i32>() {
            Ok(x) => {
                writeln!(out, "Case {}: {}", i+1, x).ok();
            }
            _ => break,
        }
    }
}
