#[allow(unused_imports)]
use std::cmp::{max, min};
use std::io::{stdin, stdout, BufWriter, Write};
use std::collections::BTreeMap;

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

struct FenwickTree {
    buf: Vec<i64>,
    size: usize,
}

impl FenwickTree {
    pub fn new(n: usize) -> FenwickTree {
        FenwickTree {
            buf: vec![0; n + 1],
            size: n,
        }
    }

    pub fn add(&mut self, i: usize, v: i64) {
        let mut i = i + 1;
        while i <= self.size {
            self.buf[i] += v;
            i += i & i.wrapping_neg();
        }
    }

    pub fn sum(&self, i: usize) -> i64 {
        let mut i = i + 1;
        let mut sum = 0;
        while i > 0 {
            sum += self.buf[i];
            i -= i & i.wrapping_neg();
        }
        sum
    }
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = sc.next();
    let mut a = vec![0; n];
    let mut map = BTreeMap::<i64, usize>::new();
    for i in 0..n {
        a[i] = sc.next();
        map.insert(a[i], 0);
    }
    for (i, (_, v)) in map.iter_mut().enumerate() {
        *v = i;
    }

    let mut ft = FenwickTree::new(n);
    let mut ans = 0;
    for x in a.iter().rev() {
        let i = map[&x];
        ans += ft.sum(i);
        // writeln!(out, "{} {}: {}", x, i, ans ).ok();
        ft.add(i, 1);
    }
    writeln!(out, "{}", ans).ok();
}
