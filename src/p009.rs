pub fn run() {
    for i in 1..=1000 {
        for j in 1..=1000 {
            for k in 1..=1000 {
                if i + j + k > 1000 {
                    break;
                }

                let p = i32::pow(i, 2);
                let q = i32::pow(j, 2);

                if p + q == i32::pow(k, 2) {
                    if i + j + k == 1000 {
                        print!("{i} {j} {k} {}", i * j * k);
                    }
                }
            }
        }
    }
}
