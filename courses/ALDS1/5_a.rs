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

fn solve(a: &[i32], m: usize) -> bool {
    let n = a.len();
    let mut dp = vec![vec![false; m+1]; n+1];
    dp[0][0] = true;
    for i in 0..n {
        for j in 0..m+1 {
            dp[i+1][j] = dp[i][j];
            if j as i32 >= a[i] {
                dp[i+1][j] |= dp[i][j - a[i] as usize]
            }
        }
    }
    dp[n][m]
}

/* Blute force
fn solve(a: &[i32], m: i32, sum: i32, ind: usize) -> bool {
    if sum == m {
        return true;
    }
    if sum > m || ind >= a.len() {
        return false;
    }
    solve (a, m, sum + a[ind], ind+1) || solve(a, m, sum, ind+1)
}
*/

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = scan.next();
    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = scan.next();
    }

    let q = scan.next();
    for _ in 0..q {
        let m = scan.next();
        if solve(&a[..], m) {
            writeln!(out, "yes").ok();
        } else {
            writeln!(out, "no").ok();
        }
    }
}
