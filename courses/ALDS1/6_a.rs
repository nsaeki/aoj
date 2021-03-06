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

fn counting_sort(a: &Vec<usize>, k: usize) -> Vec<usize> {
    let mut c = vec![0; k];
    for &x in a.iter() {
        c[x] += 1;
    }
    for i in 1..k {
        c[i] += c[i - 1];
    }
    let mut b = vec![0; a.len()];
    for &x in a.iter().rev() {
        b[c[x] - 1] = x;
        c[x] -= 1;
    }
    b
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let out = stdout();
    let mut out = out.lock();

    let n = sc.next();
    let mut a = vec![0; n];
    let mut k = 0;
    for i in 0..n {
        a[i] = sc.next();
        if k < a[i] {
            k = a[i];
        }
    }
    k += 1;

    let b = counting_sort(&a, k);
    let res = b.iter().map(ToString::to_string).collect::<Vec<String>>().join(" ");
    writeln!(out, "{}", res).ok();
}
