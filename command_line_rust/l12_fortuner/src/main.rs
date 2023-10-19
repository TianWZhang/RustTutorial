fn main() {
    if let Err(e) = l12_fortuner::get_args().and_then(l12_fortuner::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
