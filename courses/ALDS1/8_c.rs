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

struct Tree {
    nodes: Vec<TreeNode>,
    root: usize,
}

struct TreeNode {
    key: i32,
    left: usize,
    right: usize,
    parent: usize,
}

impl Tree {
    fn insert(&mut self, z: i32) {
        let i = self.nodes.len();
        let mut z = TreeNode {
            key: z,
            left: i,
            right: i,
            parent: i,
        };

        if i == 0 {
            self.root = 0;
            self.nodes.push(z);
            return;
        }

        let mut y;
        let mut x = self.root;
        loop {
            y = x;
            if z.key < self.nodes[x].key {
                if self.nodes[x].left == x {
                    break;
                }
                x = self.nodes[x].left;
            } else {
                if self.nodes[x].right == x {
                    break;
                }
                x = self.nodes[x].right;
            }
        }
        z.parent = y;
        if z.key < self.nodes[y].key {
            self.nodes[y].left = i;
        } else {
            self.nodes[y].right = i;
        }

        self.nodes.push(z);
    }

    fn find(&self, x: i32) -> Option<usize> {
        self.find_helper(self.root, 0, x)
    }

    fn find_helper(&self, i: usize, p: usize, x: i32) -> Option<usize> {
        if i != self.root && i == p {
            return None;
        }

        let k = self.nodes[i].key;
        if k == x {
            Some(i)
        } else if x < k {
            self.find_helper(self.nodes[i].left, i, x)
        } else {
            self.find_helper(self.nodes[i].right, i, x)
        }
    }

    fn delete(&mut self, x: i32) {
        if let Some(i) = self.find(x) {
            self.delete_node(i)
        }
    }

    fn delete_node(&mut self, i: usize) {
        let l = self.nodes[i].left;
        let r = self.nodes[i].right;
        let p = self.nodes[i].parent;
        let mut c = r;
        if r == i {
            c = l;
            if l == i {
                c = p;
            }
        }

        if l == i && r == i {
            self.replace_child(p, i, p);
        } else if l != i && r != i {
            let n = self.next_inorder(r);
            self.nodes[i].key = self.nodes[n].key;
            self.delete_node(n);
        } else {
            self.replace_child(p, i, c);
            self.nodes[c].parent = p;
        }
        if self.root == i {
            self.root = c;
            self.nodes[c].parent = c;
        }
    }

    fn replace_child(&mut self, p: usize, a: usize, b: usize) {
        if self.nodes[p].left == a {
            self.nodes[p].left = b;
        } else {
            self.nodes[p].right = b;
        }
    }

    fn next_inorder(&self, i: usize) -> usize {
        if self.nodes[i].left != i {
            self.next_inorder(self.nodes[i].left)
        } else {
            i
        }
    }

    fn print<T: Write>(&self, out: &mut T) {
        self.print_inorder(self.root, out);
        writeln!(out).ok();
        self.print_preorder(self.root, out);
        writeln!(out).ok();
    }

    fn print_inorder<W: Write>(&self, i: usize, out: &mut W) {
        if self.nodes[i].left != i {
            self.print_inorder(self.nodes[i].left, out);
        }
        write!(out, " {}", self.nodes[i].key).ok();
        if self.nodes[i].right != i {
            self.print_inorder(self.nodes[i].right, out);
        }
    }

    fn print_preorder<W: Write>(&self, i: usize, out: &mut W) {
        write!(out, " {}", self.nodes[i].key).ok();
        if self.nodes[i].left != i {
            self.print_preorder(self.nodes[i].left, out);
        }
        if self.nodes[i].right != i {
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
    let mut t = Tree {
        nodes: Vec::new(),
        root: 0,
    };
    for _ in 0..n {
        let op: String = sc.next();
        match op.as_str() {
            "insert" => t.insert(sc.next()),
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
