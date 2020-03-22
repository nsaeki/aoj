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
    let mut s = vec![0; n];
    for i in 0..n {
        s[i] = scan.next();
    }
    let q = scan.next();
    let mut t = vec![0; q];
    for i in 0..q {
        t[i] = scan.next();
    }

    s.sort();
    t.sort();
    let mut res = 0;
    let mut j = 0;
    for i in 0..q {
        while j < n && s[j] < t[i] {
            j += 1;
        }
        if j == n {
            break;
        }
        if s[j] == t[i] {
            res += 1;
        }
    }
    writeln!(out, "{}", res).ok();
}
