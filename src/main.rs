use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 1000)]
    steps: usize
}

fn main() {
    let args = Args::parse();

    println!("Running simulation with {} steps", args.steps);
}
