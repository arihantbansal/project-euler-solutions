pub fn run() {
    let mut order = 1;
    const EXP: u64 = 1000;

    let digits = 1_f64 + (EXP as f64) * 2_f64.ln() / 10_f64.ln();

    let mut num: Vec<u64> = Vec::new();

    num.push(1);

    for i in 0..EXP {
        let mut carry = 0;

        for j in 0..=order {
            let mut product =
                2 * (match num.get(j) {
                    Some(n) => n,
                    None => &0,
                }) + carry;

            match num.get(j) {
                Some(n) => {
                    num[j] = product % 10;
                }
                None => num.push(product % 10),
            }

            carry = product / 10 | 0;

            if j == order && carry > 0 {
                order += 1;
            }
        }
    }

    println!("{}", num.iter().sum::<u64>());
}
