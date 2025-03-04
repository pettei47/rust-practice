use clap::Parser;
use fortuner::Config;

fn main() {
    let config = Config::parse();

    if let Err(e) = fortuner::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
