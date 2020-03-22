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

    let n: i32 = scan.next();
    let mut a = [[[0; 10]; 3]; 4];
    for _ in 0..n {
        let b: usize = scan.next();
        let f: usize = scan.next();
        let r: usize = scan.next();
        let v: i32 = scan.next();
        a[b-1][f-1][r-1] += v;
    }
    for b in 0..4 {
        for f in 0..3 {
            for r in 0..10 {
                write!(out, " {}", a[b][f][r]).ok();
            }
            writeln!(out, "").ok();
        }
        if b < 3 {
            writeln!(out, "####################").ok();
        }
    }
}
