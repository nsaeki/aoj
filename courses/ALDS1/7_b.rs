#[allow(unused_imports)]
use std::cmp::{max, min};
use std::collections::VecDeque;
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
    sibling: usize,
    degree: usize,
    depth: usize,
    height: usize,
    left: usize,
    right: usize,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let t = if self.depth == 0 {
            "root"
        } else if self.height == 0 {
            "leaf"
        } else {
            "internal node"
        };

        let mut p = self.parent as i32;
        if self.parent == self.id {
            p = -1;
        }

        let mut sib = self.sibling as i32;
        if self.sibling == self.id {
            sib = -1;
        }

        write!(
            f,
            "node {}: parent = {}, sibling = {}, degree = {}, depth = {}, height = {}, {}",
            self.id, p, sib, self.degree, self.depth, self.height, t
        )
    }
}

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
            sibling: i,
            degree: 0,
            depth: 0,
            height: 0,
            left: i,
            right: i,
        })
    }

    for _ in 0..n {
        let i: usize = sc.next();
        let l: i32 = sc.next();
        let r: i32 = sc.next();
        if l != -1 {
            let l = l as usize;
            node[i].degree += 1;
            node[i].left = l;
            node[l].parent = i;
        }
        if r != -1 {
            let r = r as usize;
            node[i].degree += 1;
            node[i].right = r;
            node[r].parent = i;
        }
        if r != -1 && l != -1 {
            let l = l as usize;
            let r = r as usize;
            node[l].sibling = r;
            node[r].sibling = l;
        }
    }

    let root = node.iter().find(|x| x.id == x.parent).unwrap().id;
    let mut stack = vec![(root, 0)];
    let mut queue = VecDeque::<(usize, usize)>::new();
    while !stack.is_empty() {
        let (i, d) = stack.pop().unwrap();
        node[i].depth = d;
        if node[i].left != i {
            stack.push((node[i].left, d + 1));
        }
        if node[i].right != i {
            stack.push((node[i].right, d + 1));
        }
        if node[i].left == i && node[i].right == i {
            queue.push_back((i, 0));
        }
    }

    while !queue.is_empty() {
        let (i, h) = queue.pop_front().unwrap();
        node[i].height = h;
        if node[i].parent != node[i].id {
            queue.push_back((node[i].parent, h + 1));
        }
    }

    for x in node.iter() {
        writeln!(out, "{}", x).ok();
    }
}
