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

struct Node {
    id: usize,
    parent: usize,
    left: usize,
    right: usize,
}

fn dfs_preorder<T: Write>(i: usize, node: &[Node], out: &mut T) {
    write!(out, " {}", i).ok();
    if node[i].left != i {
        dfs_preorder(node[i].left, node, out);
    }
    if node[i].right != i {
        dfs_preorder(node[i].right, node, out);
    }
}

fn dfs_inorder<T: Write>(i: usize, node: &[Node], out: &mut T) {
    if node[i].left != i {
        dfs_inorder(node[i].left, node, out);
    }
    write!(out, " {}", i).ok();
    if node[i].right != i {
        dfs_inorder(node[i].right, node, out);
    }
}

fn dfs_postorder<T: Write>(i: usize, node: &[Node], out: &mut T) {
    if node[i].left != i {
        dfs_postorder(node[i].left, node, out);
    }
    if node[i].right != i {
        dfs_postorder(node[i].right, node, out);
    }
    write!(out, " {}", i).ok();
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
            node[i].left = l;
            node[l].parent = i;
        }
        if r != -1 {
            let r = r as usize;
            node[i].right = r;
            node[r].parent = i;
        }
    }

    let root = node.iter().find(|x| x.id == x.parent).unwrap().id;
    writeln!(out, "Preorder").ok();
    dfs_preorder(root, &node, &mut out);
    writeln!(out).ok();
    writeln!(out, "Inorder").ok();
    dfs_inorder(root, &node, &mut out);
    writeln!(out).ok();    writeln!(out, "Postorder").ok();
    dfs_postorder(root, &node, &mut out);
    writeln!(out).ok();
}
