pub fn run() {
    let mut line = String::new();
    println!("Enter input: ");
    std::io::stdin().read_line(&mut line).unwrap();

    let mut max_prod: u64 = 0;

    for i in 0..(line.len() - 13) {
        let mut product: u64 = 1;

        for j in 0..13 {
            let num: u64 = line
                .chars()
                .nth(i + j)
                .unwrap()
                .to_digit(10)
                .unwrap()
                .into();

            product *= num;
        }

        max_prod = u64::max(max_prod, product.into());
    }

    print!("{}", max_prod);
}
