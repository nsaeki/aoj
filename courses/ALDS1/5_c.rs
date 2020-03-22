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

type P = (f64, f64);
fn rotate(p1: P, p2: P, rad: f64) -> P {
    (
        p1.0 + (p2.0 - p1.0) * rad.cos() - (p2.1 - p1.1) * rad.sin(),
        p1.1 + (p2.0 - p1.0) * rad.sin() + (p2.1 - p1.1) * rad.cos(),
    )
}

fn koch(mut v: &mut Vec<P>, p1: P, p2: P, lv: usize) {
    if lv == 0 {
        v.push(p1);
        return;
    }
    let l = ((p2.0 - p1.0) / 3.0, (p2.1 - p1.1) / 3.0);
    let s = (p1.0 + l.0, p1.1 + l.1);
    let t = (p1.0 + l.0 * 2.0, p1.1 + l.1 * 2.0);
    let rad = std::f64::consts::PI / 3.0;
    let u = rotate(s, t, rad);
    koch(&mut v, p1, s, lv - 1);
    koch(&mut v, s, u, lv - 1);
    koch(&mut v, u, t, lv - 1);
    koch(&mut v, t, p2, lv - 1);
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());
    let n = sc.next();
    let mut ans = vec![];
    koch(&mut ans, (0.0, 0.0), (100.0, 0.0), n);
    ans.push((100.0, 0.0));
    for p in &ans {
        writeln!(out, "{} {}", p.0, p.1).ok();
    }
}
