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
    let n = scan.next::<i32>();
    for i in 1..n+1 {
        let mut x = i;
        if x % 3 == 0 {
            write!(out, " {}", i).ok();
            continue;
        }
        while x > 0 {
            if x % 10 == 3 {
                write!(out, " {}", i).ok();
                break;
            }
            x /= 10;
        }
    }
    writeln!(out).ok();
}
