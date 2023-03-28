pub fn run() {
    let n: i64 = 600851475143;
    let mut max: i64 = 0;

    let largest_possible = ((n as f64).sqrt() + 1.0) as i64;

    for i in (3..largest_possible).rev() {
        if i % 1_000_000_000 == 0 {
            println!("{}", i);
        }

        if i % 2 == 0 {
            continue;
        }

        if n % i == 0 && is_prime(i) {
            max = std::cmp::max(max, i);
        }
    }

    println!("{}", max);
}

fn is_prime(n: i64) -> bool {
    let sum: i64 = (1..n).filter(|i| n % i == 0).sum();

    sum == 1
}
