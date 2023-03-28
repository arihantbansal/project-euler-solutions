use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    problem_number: u8,
}

fn main() {
    let args = Args::parse();

    println!("You ran problem #{:?}", args.problem_number);
}
