use clap::Parser;
use tailr::Config;

fn main() {
    let config = Config::parse();

    if let Err(e) = tailr::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
