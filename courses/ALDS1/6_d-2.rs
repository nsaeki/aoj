#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::io::{stdin, stdout, Read, Write};
use std::str::FromStr;

#[derive(Default)]
struct Scanner<R: Read> {
    reader: R,
}

impl<R: Read> Scanner<R> {
    fn new(reader: R) -> Scanner<R> {
        Scanner { reader }
    }

    fn scan<T: FromStr>(&mut self) -> Option<T> {
        let s = self
            .reader
            .by_ref()
            .bytes()
            .map(|c| c.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        s.parse::<T>().ok()
    }

    fn next<T: FromStr>(&mut self) -> T {
        if let Some(s) = self.scan() {
            s
        } else {
            std::process::exit(1);
        }
    }
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let out = stdout();
    let mut out = out.lock();

    let n = sc.next();
    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = sc.next();
    }

    let a_min = a.iter().min().unwrap();
    let mut b = vec![0; n];
    for i in 0..n {
        b[i] = a.iter().filter(|&&x| x < a[i]).count();
    }

    let mut seen = vec![false; n];
    let mut ans = 0;
    for i in 0..n {
        if seen[i] || b[i] == i {
            continue;
        }
        let mut v = Vec::<usize>::new();
        let mut i = i;
        while !seen[i] {
            seen[i] = true;
            v.push(a[i]);
            i = b[i];
        }
        v.sort();
        let sum = v[1..].iter().fold(0, |sum, x| sum + x);
        ans += min(
            v[0] * (v.len() - 1) + sum,
            a_min * (v.len() - 1) + sum + (v[0] + a_min) * 2,
        )
    }

    writeln!(out, "{}", ans).ok();
}
