use reikna::factor::quick_factorize;

// Solution #1
pub fn run() {
    let mut i = 1;
    let mut triangle = 0;

    loop {
        triangle += i;
        let factors = quick_factorize(triangle);

        let freq = freqs(&factors, triangle.try_into().unwrap());

        println!("{:?}", freq);

        let num_factors = freq.iter().fold(1, |acc, e| e * (acc + 1));

        println!("{i} {triangle} {num_factors}");

        if num_factors > 5 {
            println!("{} has {} factors", i, num_factors);
            break;
        }
        i += 1;
    }
}

fn freqs(data: &[u64], size: usize) -> Vec<u32> {
    data.iter().fold(vec![0_u32; size + 1], |mut freqs, &el| {
        freqs[el as usize] += 1;
        freqs
    })
}

// Solution #2
// fn num_divisors(n: u64) -> u64 {
//     let mut result = 0;
//     let max = (n as f64).sqrt() as u64;
//     for i in 1..max + 1 {
//         if n % i == 0 {
//             if n / i == i {
//                 result += 1;
//             } else {
//                 result += 2;
//             }
//         }
//     }
//     return result;
// }

// pub fn run() {
//     let triangles = (1..).scan(0, |triangle, n| {
//         *triangle += n;
//         Some(*triangle)
//     });
//     let first = triangles
//         .skip_while(|&triangle| num_divisors(triangle) <= 500)
//         .next()
//         .unwrap();
//     println!("{}", first);
// }
