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
        let mut z = TreeNode{key: z, left: i, right: i, parent: i};

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

    fn find(&self, x: i32) -> bool {
        self.find_helper(self.root, x)
    }

    fn find_helper(&self, i: usize, x: i32) -> bool {
        let k = self.nodes[i].key;
        // match x {
        //     x => true,
        //     ..k => self.nodes[i].left == i && self.find_helper(self.nodes[i].left, x),
        //     k.. => self.nodes[i].right == i && self.find_helper(self.nodes[i].right, x),
        // }
        if k == x {
            true
        } else if x < k && self.nodes[i].left != i {
            self.find_helper(self.nodes[i].left, x)
        } else if x > k && self.nodes[i].right != i {
            self.find_helper(self.nodes[i].right, x)
        } else {
            false
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
    let mut t = Tree{nodes: Vec::new(), root: 0};
    for _ in 0..n {
        let op: String = sc.next();
        match op.as_str() {
            "insert" => t.insert(sc.next()),
            "print" => t.print(&mut out),
            "find" => {
                if t.find(sc.next()) {
                    writeln!(out, "yes").ok();
                } else {
                    writeln!(out, "no").ok();
                }
            },
            _  => panic!("unknown operatiion {}", op),
        }
    }
}
