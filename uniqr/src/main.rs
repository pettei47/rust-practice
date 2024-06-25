use clap::Parser;
use uniqr::Config;

fn main() {
    let config = Config::parse();

    if let Err(e) = uniqr::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
