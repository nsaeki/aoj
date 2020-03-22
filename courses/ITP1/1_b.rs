fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let x = s.trim().parse::<i64>().ok().unwrap();

    println!("{}", x*x*x);
}