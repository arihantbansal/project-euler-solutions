fn main() {
    let n = 10_001;
    let max_bound = upper_bound_for_p_n(n);

    println!("Upper bound for {} primes is {}", n, max_bound);

    let mut count = 0;

    for i in 2..max_bound {
        if is_prime(i) {
            count += 1;
        }
        if count == n {
            println!("{i} is the {}th prime", count);
            break;
        }
        if i % 1000 == 0 {
            println!("Checked {}, found {} primes", i, count);
        }
    }
}

fn upper_bound_for_p_n(n: i32) -> i64 {
    if n < 6 {
        return 100;
    }
    return f64::ceil((n as f64) * (f64::ln(n.into()) + f64::ln(f64::ln(n.into())))) as i64;
}

fn is_prime(n: i64) -> bool {
    let sum: i64 = (1..n).filter(|i| n % i == 0).sum();

    sum == 1
}
