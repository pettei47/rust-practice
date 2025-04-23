use clap::Parser;
use calr::Config;

fn main() {
    let config = Config::parse();

    if let Err(e) = calr::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
