#[allow(unused_imports)]
use std::cmp::{max, min};
use std::io::{stdin, stdout, BufWriter, Write};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let w = scan.next::<i32>();
    let h = scan.next::<i32>();
    let x = scan.next::<i32>();
    let y = scan.next::<i32>();
    let r = scan.next::<i32>();

    if x - r < 0 || x + r > w || y - r < 0 || y + r > h {
        writeln!(out, "No").ok();
    } else {
        writeln!(out, "Yes").ok();
    }
}
