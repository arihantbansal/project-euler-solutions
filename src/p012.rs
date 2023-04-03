pub fn run() {
    let mut i = 1_000_000;
    loop {
        let triangle = (i + 1) * i / 2;
        let factors = get_factors_functional(triangle).len();
        println!("{i} has {factors}");
        if factors > 500 {
            println!("{} has {} factors", i, factors);
            break;
        }
        i += 1;
    }
}

fn get_factors_functional(n: u64) -> Vec<u64> {
    (1..n + 1)
        .into_iter()
        .filter(|&x| n % x == 0)
        .collect::<Vec<u64>>()
}
