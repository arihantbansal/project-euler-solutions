pub fn run() {
    let mut ans = lcm(1, 2);

    for i in 3..=20 {
        ans = lcm(i, ans);
    }

    println!("{}", ans);
}

fn lcm(a: i32, b: i32) -> i32 {
    (a / gcd(a, b)) * b
}

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 {
        return b;
    }
    gcd(b % a, a)
}
