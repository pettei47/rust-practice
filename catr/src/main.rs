fn main() {
    if let Err(e) = catr::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
