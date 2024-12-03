use clap::Parser;
use commr::Config;

fn main() {
    let config = Config::parse();

    if let Err(e) = commr::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
