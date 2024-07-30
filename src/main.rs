fn main() {
    if let Err(e) = soncat::get_args().and_then(soncat::run) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
