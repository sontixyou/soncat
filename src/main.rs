fn main() {
    if let Err(e) = soncat::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
