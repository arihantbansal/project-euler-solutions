fn main() {
    let n = 100;
    let ans = i32::pow(n * (n + 1) / 2, 2) - (n * (n + 1) * (2 * n + 1) / 6);
    println!("{}", ans);
}
