use catr::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    // dbg!(args);
    if let Err(e) = catr::run(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
