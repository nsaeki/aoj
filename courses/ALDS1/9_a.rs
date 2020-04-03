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

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let out = stdout();
    let mut out = out.lock();

    let n: usize = sc.next();
    let mut a = vec![0; n+1];
    for i in 1..(n+1) {
        a[i] = sc.next();
    }
    for i in 1..(n+1) {
        let mut s = format!("node {}: key = {}, ", i, a[i]);
        if i/2 > 0 {
            s += format!("parent key = {}, ", a[i/2]).as_str();
        }
        if i*2 <= n {
            s += format!("left key = {}, ", a[i*2]).as_str();
        }
        if i*2 + 1 <= n {
            s +=  format!("right key = {}, ", a[i*2+1]).as_str();
        }
        writeln!(out, "{}", s).ok();
    }
}