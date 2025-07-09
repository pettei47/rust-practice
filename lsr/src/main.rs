use clap::Parser;
use lsr::Config;

fn main() {
    let config = Config::parse();

    if let Err(e) = lsr::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
