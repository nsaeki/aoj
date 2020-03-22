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
    loop {
        let h:i32 = scan.next();
        let w:i32 = scan.next();
        if h == 0 && w == 0 { break; }
        for i in 0..h {
            for j in 0..w {
                if (i+j) & 1 == 0 {
                    write!(out, "#").ok();
                } else {
                    write!(out, ".").ok();
                }
            }
            writeln!(out).ok();
        }
        writeln!(out).ok();
    }
}