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

fn max_heapify(a: &mut[i32], i: usize) {
    let l = i * 2;
    let r = i * 2 + 1;
    let mut largest = i;
    if l < a.len() && a[l] > a[i] {
        largest = l;
    }
    if r < a.len() && a[r] > a[largest] {
        largest = r;
    }
    if largest != i {
        a.swap(i, largest);
        max_heapify(a, largest);
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
    for i in 1..n+1 {
        a[i] = sc.next();
    }
    for i in (1..(n/2+1)).rev() {
        max_heapify(a.as_mut_slice(), i);
    }
    for i in 1..n+1 {
        write!(out, " {}", a[i]).ok();
    }
    writeln!(out).ok();
}