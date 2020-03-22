#[allow(unused_imports)]
use std::cmp::{max, min};
use std::io::{stdout, BufWriter, Write};

fn main() {
    let out = &mut BufWriter::new(stdout());

    for _ in 0..1000 {
        writeln!(out, "Hello World").ok();
    }    
}
