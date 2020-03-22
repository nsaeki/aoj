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

    let (a, b, c) = (scan.next::<i32>() ,scan.next::<i32>(), scan.next::<i32>());
    let mut ans = 0;
    // for i in a..=b {
    for i in a..b+1 {
        if c % i == 0 {
            ans += 1;
        }
    }
    writeln!(out, "{}", ans).ok();
}
