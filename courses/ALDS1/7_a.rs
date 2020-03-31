#[allow(unused_imports)]
use std::cmp::{max, min};
use std::fmt;
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

#[derive(Default)]
struct Node {
    id: usize,
    parent: usize,
    depth: usize,
    children: Vec<usize>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let t = if self.depth == 0 {
            "root"
        } else if self.children.len() == 0 {
            "leaf"
        } else {
            "internal node"
        };

        let mut p = self.parent as i32;
        if self.parent == self.id {
            p = -1;
        }

        write!(
            f,
            "node {}: parent = {}, depth = {}, {}, {:?}",
            self.id, p, self.depth, t, self.children
        )
    }
}

/*
fn dfs(i: usize, d: usize, n: &mut [Node]) {
    n[i].depth = d;
    for &j in n[i].children.iter() {
        dfs(j, d+1, n);
    }
}
*/
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let out = stdout();
    let mut out = out.lock();

    let n = sc.next();
    let mut node = Vec::<Node>::new();
    for i in 0..n {
        node.push(Node {
            id: i,
            parent: i,
            depth: 0,
            children: Vec::new(),
        })
    }
    for _ in 0..n {
        let id: usize = sc.next();
        let k = sc.next();
        for _ in 0..k {
            let c = sc.next();
            node[id].children.push(c);
            node[c].parent = id;
        }
    }

    let mut root = 0;
    for x in node.iter() {
        if x.parent == x.id {
            root = x.id;
            break;
        }
    }

    let mut stack = vec![(root, 0)];
    while !stack.is_empty() {
        let (i, d) = stack.pop().unwrap();
        node[i].depth = d;
        for &c in node[i].children.iter() {
            stack.push((c, d+1));
        }
    }

    for x in node.iter() {
        writeln!(out, "{}", x).ok();
    }
}
