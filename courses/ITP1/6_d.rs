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
    let n = scan.next();
    let m = scan.next();
    let mut a = vec![vec![0; m]; n];
    let mut v = vec![0; m];

    for i in 0..n {
        for j in 0..m {
            a[i][j] = scan.next();
        }
    }
    for i in 0..m {
        v[i] = scan.next();
    }

    for i in 0..n {
        let mut ret = 0;
        for j in 0..m {
            ret += a[i][j] * v[j];
        }
        writeln!(out, "{}", ret).ok();
    }
}
