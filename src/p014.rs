use std::collections::HashMap;

pub fn run() {
    const LARGEST_UNDER: u64 = 10;

    let mut counts = HashMap::<u64, u64>::new();

    for i in 1..LARGEST_UNDER {
        let mut n = i;
        let mut steps = 0;
        while n != 1 {
            println!("inside loop {i} {n} {steps}");
            steps += 1;

            if counts.contains_key(&n) {
                counts.insert(i, counts.get(&n).unwrap() + steps);
                break;
            }

            if n % 2 == 1 {
                n = 3 * n + 1;
            } else {
                n /= 2;
            }
        }

        if !counts.contains_key(&i) {
            counts.insert(i, steps + 1);
        }
    }

    for (key, value) in &counts {
        println!("{} / {}", key, value);
    }
}
