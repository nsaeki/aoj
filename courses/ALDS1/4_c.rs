#[allow(unused_imports)]
use std::cmp::{max, min};
use std::io::{stdin, stdout, BufWriter, Write};
use std::collections::HashMap;

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
    let n = scan.next();
    let mut dict = HashMap::new();

    for _i in 0..n {
        let op: String = scan.next();
        let k: String = scan.next();
        if op == "insert" {
            dict.insert(k, true);
        } else {
            if dict.contains_key(&k) {
                writeln!(out, "yes").ok();
            } else {
                writeln!(out, "no").ok();
            }
        }
    }
}
