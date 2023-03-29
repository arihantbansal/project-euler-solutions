use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    problem_number: u8,
}

fn main() {
    let args = Args::parse();

    println!("You ran problem #{:?}", args.problem_number);
}
