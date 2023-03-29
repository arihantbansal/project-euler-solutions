use clap::Parser;

mod p001;
mod p002;
mod p003;
mod p004;
mod p005;
mod p006;
mod p007;
mod p008;

#[derive(Parser, Debug)]
struct Args {
    problem_number: u8,
}

fn main() {
    let args = Args::parse();

    println!("You ran problem #{:?}", args.problem_number);

    match args.problem_number {
        1 => p001::run(),
        2 => p002::run(),
        3 => p003::run(),
        4 => p004::run(),
        5 => p005::run(),
        6 => p006::run(),
        7 => p007::run(),
        8 => p008::run(),
        _ => println!("Problem hasn't been solved yet :("),
    }
}
