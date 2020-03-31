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

fn dfs(pre_order: &[usize], in_order: &[usize]) -> Vec<usize> {
    assert_eq!(pre_order.len(), in_order.len());
    if pre_order.len() == 0 {
        return Vec::new();
    }
    let r = pre_order[0];
    let i = in_order.iter().position(|&x| x == r).unwrap();
    let mut v = Vec::<usize>::new();
    if i > 0 {
        let mut ret = dfs(&pre_order[1..i+1], &in_order[0..i]);
        v.append(&mut ret);
    }
    if i < pre_order.len() {
        let mut ret = dfs(&pre_order[i+1..], &in_order[i+1..]);
        v.append(&mut ret);
    }
    v.push(r);
    v
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let out = stdout();
    let mut out = out.lock();

    let n = sc.next();
    let pre_order = (0..n).map(|_| sc.next()).collect::<Vec<usize>>();
    let in_order = (0..n).map(|_| sc.next()).collect::<Vec<usize>>();

    let v = dfs(&pre_order[..], &in_order[..]);
    writeln!(out, "{}", v.iter().map(ToString::to_string).collect::<Vec<String>>().join(" ")).ok();
}