use clap::Parser;
use grepr::Config;

fn main() {
    let config = Config::parse();

    if let Err(e) = grepr::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
