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

fn bsearch(w: &[i32], k: i32, l: i32, r: i32) -> i32 {
    let m = (l + r) / 2;
    if l == m {
        if can_load(w, k, l) {
            return l;
        } else {
            return r;
        }
    }
    if can_load(w, k, m) {
        bsearch(w, k, l, m)
    } else {
        bsearch(w, k, m, r)
    }
}

fn can_load(w: &[i32], k: i32, p: i32) -> bool {
    let mut i = 0;
    for _ in 0..k {
        let mut load = 0;
        while i < w.len() {
            if load + w[i] > p {
                break;
            }
            load += w[i];
            i += 1;
        }
    }
    i == w.len()
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());
    let n = scan.next();
    let k = scan.next();
    let mut w = vec![0; n];
    for i in 0..n {
        w[i] = scan.next();
    }

    writeln!(out, "{}", bsearch(&w[..], k, 0, 10_000 * 100_000)).ok();
}
