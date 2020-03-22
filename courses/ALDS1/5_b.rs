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

fn merge(a: &mut [i32], left: usize, mid: usize, right: usize) -> i32{
    let n1 = mid - left;
    let n2 = right - mid;
    let mut l = vec![0; n1];
    let mut r = vec![0; n2];
    l.copy_from_slice(&a[left..mid]);
    r.copy_from_slice(&a[mid..right]);
    l.push(std::i32::MAX);
    r.push(std::i32::MAX);
    let mut i = 0;
    let mut j = 0;
    let mut cnt = 0;
    for k in left..right {
        cnt+=1;
        if l[i] <= r[j] {
            a[k] = l[i];
            i += 1;
        } else {
            a[k] = r[j];
            j += 1;
        }
    }
    cnt
}

fn merge_sort(a: &mut[i32], left: usize, right: usize) -> i32{
    let mut cnt = 0;
    if left + 1 < right {
        let mid = (left + right) / 2;
        cnt += merge_sort(a, left, mid);
        cnt += merge_sort(a, mid, right);
        cnt += merge(a, left, mid, right);
    }
    cnt
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());
    let n = sc.next();
    let mut a = Vec::new();
    for _ in 0..n {
        let x: i32 = sc.next();
        a.push(x);
    }
    let cnt = merge_sort(&mut a[..], 0, n);
    let a = a.iter().map(ToString::to_string).collect::<Vec<String>>().join(" ");
    writeln!(out, "{}", a).ok();
    writeln!(out, "{}", cnt).ok();
}
