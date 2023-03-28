pub fn run() {
    let n = 1000;
    let sum: i32 = (1..n).filter(|i| i % 3 == 0 || i % 5 == 0).sum();

    println!("{}", sum);
}
