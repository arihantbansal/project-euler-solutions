use primes::{PrimeSet, Sieve};

pub fn run() {
    let mut pset = Sieve::new();

    let mut sum: u64 = 0;

    for (_ix, n) in pset.iter().enumerate() {
        if n > 2_000_000 {
            break;
        }
        sum += n;
    }

    println!("{sum}");
}
