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

fn bsearch(x: i32, s: &[i32]) -> bool {
    if s.len() == 1 {
        if x == s[0] {
            return true;
        }
        return false;
    }

    let m = s.len()/2;
    if x < s[m] {
        bsearch(x, &s[..m])
    } else {
        bsearch(x, &s[m..])
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
    let mut t = vec![0; n];
    for i in 0..q {
        t[i] = scan.next()
    }

    s.sort();
    let mut res = 0;
    for i in 0..q {
        if bsearch(t[i], &s) {
            res += 1;
        }
    }
    writeln!(out, "{}", res).ok();
}
