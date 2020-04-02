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

struct Treap {
    nodes: Vec<TreapNode>,
    root: Option<usize>,
}

struct TreapNode {
    key: i32,
    priority: i32,
    left: Option<usize>,
    right: Option<usize>,
}

impl Treap {
    fn new() -> Treap {
        Treap { nodes: Vec::new(), root: None }
    }

    fn insert(&mut self, key: i32, priority: i32) {
        let root = self.root;
        self.root = self.insert_helper(root, key, priority);
    }

    fn insert_helper(&mut self, t: Option<usize>, key: i32, priority: i32) -> Option<usize> {
        if t == None {
            self.nodes.push(TreapNode{key, priority, left: t, right: t});
            return Some(self.nodes.len() - 1);
        }

        let mut t = t.unwrap();
        if key < self.nodes[t].key {
            let mut l = self.nodes[t].left;
            l = self.insert_helper(l, key, priority);
            self.nodes[t].left = l;
            let l = l.unwrap();
            if self.nodes[t].priority < self.nodes[l].priority {
                t = self.rotate_right(t);
            }
        } else {
            let mut r = self.nodes[t].right;
            r = self.insert_helper(r, key, priority);
            self.nodes[t].right = r;
            let r = r.unwrap();
            if self.nodes[t].priority < self.nodes[r].priority {
                t = self.rotate_left(t);
            }
        }
        Some(t)
    }

    fn rotate_right(&mut self, t: usize) -> usize {
        let s = self.nodes[t].left.unwrap();
        self.nodes[t].left = self.nodes[s].right;
        self.nodes[s].right = Some(t);
        s
    }

    fn rotate_left(&mut self, t: usize) -> usize {
        let s = self.nodes[t].right.unwrap();
        self.nodes[t].right = self.nodes[s].left;
        self.nodes[s].left = Some(t);
        s
    }

    fn find(&self, key: i32) -> Option<usize> {
        self.find_helper(self.root, key)
    }

    fn find_helper(&self, t: Option<usize>, x: i32) -> Option<usize> {
        if t == None {
            return None;
        }

        let t = t.unwrap();
        let k = self.nodes[t].key;
        if k == x {
            Some(t)
        } else if x < k {
            self.find_helper(self.nodes[t].left, x)
        } else {
            self.find_helper(self.nodes[t].right, x)
        }
    }

    fn delete(&mut self, key: i32) {
        let root = self.root;
        self.root = self.delete_helper(root, key);
    }

    fn delete_helper(&mut self, t: Option<usize>, key: i32) -> Option<usize> {
        if t == None {
            return None;
        }
        let t = t.unwrap();
        if key < self.nodes[t].key {
            let l = self.nodes[t].left;
            self.nodes[t].left = self.delete_helper(l, key);
        } else if key > self.nodes[t].key {
            let r = self.nodes[t].right;
            self.nodes[t].right = self.delete_helper(r, key);
        } else {
            return self.delete_node(t, key);
        }
        Some(t)
    }

    fn delete_node(&mut self, t: usize, key: i32) -> Option<usize> {
        let mut t = t;
        let l = self.nodes[t].left;
        let r = self.nodes[t].right;
        if l == None && r == None {
            return None;
        } else if l == None {
            t = self.rotate_left(t);
        } else if r == None {
            t = self.rotate_right(t);
        } else {
            let l = l.unwrap();
            let r = r.unwrap();
            if self.nodes[l].priority > self.nodes[r].priority {
                t = self.rotate_right(t);
            } else {
                t = self.rotate_left(t);
            }
        }
        self.delete_helper(Some(t), key)
    }

    fn print<T: Write>(&self, out: &mut T) {
        self.print_inorder(self.root, out);
        writeln!(out).ok();
        self.print_preorder(self.root, out);
        writeln!(out).ok();
    }

    fn print_inorder<W: Write>(&self, t: Option<usize>, out: &mut W) {
        if let Some(i) = t {
            self.print_inorder(self.nodes[i].left, out);
            write!(out, " {}", self.nodes[i].key).ok();
            self.print_inorder(self.nodes[i].right, out);
        }
    }

    fn print_preorder<W: Write>(&self, t: Option<usize>, out: &mut W) {
        if let Some(i) = t {
            write!(out, " {}", self.nodes[i].key).ok();
            self.print_preorder(self.nodes[i].left, out);
            self.print_preorder(self.nodes[i].right, out);
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
    let mut t = Treap::new();
    for _ in 0..n {
        let op: String = sc.next();
        match op.as_str() {
            "insert" => t.insert(sc.next(), sc.next()),
            "print" => t.print(&mut out),
            "find" => {
                if let Some(_) = t.find(sc.next()) {
                    writeln!(out, "yes").ok();
                } else {
                    writeln!(out, "no").ok();
                }
            }
            "delete" => t.delete(sc.next()),
            _ => panic!("unknown operation {}", op),
        }
    }
}