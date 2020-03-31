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
        Scanner {
            reader,
        }
    }

    fn scan<T: FromStr>(&mut self) -> Option<T> {
        let s = self.reader.by_ref().bytes().map(|c| c.unwrap() as char)
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

fn partition(a: &mut[i32], p: usize, r: usize) -> usize {
    let x = a[r];
    let mut i = p;
    for j in p..r {
        if a[j] <= x {
            a.swap(i, j);
            i += 1;
        }
    }
    a.swap(i, r);
    i
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

    let q = partition(&mut a[..], 0, n-1);
    for i in 0..n {
        if i == q {
            write!(out, "[{}]", a[i]).ok();
        } else {
            write!(out, "{}", a[i]).ok();
        }
        if i != n-1 {
            write!(out, " ").ok();
        } else {
            write!(out, "\n").ok();
        }
    }
}
