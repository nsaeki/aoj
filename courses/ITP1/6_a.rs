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
    let n:usize = scan.next();
    let mut a = Vec::<i32>::new();
    for _ in 0..n {
        let x:i32 = scan.next();
        a.push(x);
    }

    write!(out, "{}", a[n-1]).ok();
    for i in 2..n+1 {
        write!(out, " {}", a[n-i]).ok();
    }
    writeln!(out).ok();
}
