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

    let mut m = BTreeMap::<usize, usize>::new();
    for (i, &x) in a.iter().enumerate() {
        m.insert(x, i);
    }

    a.sort();
    let mut grp = Vec::<Vec<usize>>::new();
    let mut seen = vec![false; n];
    for (j, (&x, &i)) in m.iter().enumerate() {
        if seen[j] {
            continue;
        }
        seen[j] = true;
        grp.push(vec![x]);
        let mut i = i;
        while !seen[i] {
            seen[i] = true;
            grp.last_mut().unwrap().push(a[i]);
            i = *m.get(&a[i]).unwrap();
        }
    }

    let mut ans = 0;
    for g in grp.iter() {
        // g[0]は作りからしてグループ内の最小値になっているのでソートはしない
        // g.sort();
        let sum = g[1..].iter().fold(0, |sum, x| sum + x);
        let cost1 = g[0] * (g.len() - 1) + sum;
        let cost2 = a[0] * (g.len() - 1) + sum + (a[0] + g[0]) * 2;
        ans += min(cost1, cost2);
    }
    writeln!(out, "{}", ans).ok();
}
