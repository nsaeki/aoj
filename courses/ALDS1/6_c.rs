#[allow(unused_imports)]
use std::cmp::{max, min};
use std::collections::BTreeMap;
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

fn partition(a: &mut [Card], p: usize, r: usize) -> usize {
    let x = a[r].v;
    let mut i = p;
    for j in p..r {
        if a[j].v <= x {
            a.swap(i, j);
            i += 1;
        }
    }
    a.swap(i, r);
    i
}

fn quicksort(a: &mut [Card], p: usize, r: usize) {
    if p < r {
        let q = partition(a, p, r);
        quicksort(a, p, q - 1);
        quicksort(a, q, r);
    }
}

struct Card {
    suit: char,
    v: i32,
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let out = stdout();
    let mut out = out.lock();

    let n = sc.next();
    let mut a = Vec::new();
    let mut map = BTreeMap::<i32, Vec<char>>::new();
    for _ in 0..n {
        let s: String = sc.next();
        let suit = s.chars().next().unwrap();
        let v = sc.next();
        let c = Card { suit, v };
        a.push(c);

        if !map.contains_key(&v) {
            map.insert(v, Vec::new());
        }
        map.get_mut(&v).unwrap().push(suit);
    }

    quicksort(&mut a[..], 0, n - 1);
    let mut i = 0;
    'outer: for (_, v) in map.iter() {
        for s in v.iter() {
            if a[i].suit != *s {
                break 'outer;
            }
            i += 1;
        }
    }
    if i == n {
        writeln!(out, "Stable").ok();
    } else {
        writeln!(out, "Not stable").ok();
    }
    for i in 0..n {
        writeln!(out, "{} {}", a[i].suit, a[i].v).ok();
    }
}
