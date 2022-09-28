fn main() {
    let mut max = 0;
    for i in (100..999).rev() {
        for j in (100..999).rev() {
            let num = i * j;
            if num == reverse(num, num.to_string().len() as u32) {
                max = std::cmp::max(max, num);
                println!("{} x {} = {}", i, j, num);
            }
        }
    }
    println!("\nMax = {}", max);
}

fn reverse(num: i32, len: u32) -> i32 {
    let x: i32 = 10;
    if len == 1 {
        return num;
    } else {
        return (num % 10) * x.pow(len - 1) + reverse(num / 10, len - 1);
    }
}
