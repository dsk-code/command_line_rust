use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Rust版echo", long_about = None)]
struct Args {
    test: String,
    #[arg(short, long, help = "input name", required = true)]
    name: String,
    #[arg(short, long, default_value_t = 1)]
    count: u8,
    #[arg(short = 'p', long)]
    check: bool,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.test);
        println!("check = {}", args.check);
    }

    println!("{:#?}", args);
}
