use clap::Parser;
use cutr::Config;

fn main() {
    let Config::parse();

    if let Err(e) = cutr::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
