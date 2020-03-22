#[allow(unused_imports)]
use std::cmp::{max, min};
use std::io::{stdin, stdout, BufWriter, Write};
use std::mem;

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

    loop {
        let mut x = scan.next::<i32>();
        let mut y = scan.next::<i32>();
        if x == 0 && y == 0 {
            break;
        }
        if x > y {
            mem::swap(&mut x, &mut y);
        }
        writeln!(out, "{} {}", x, y).ok();
    }
}
