fn main() {
    if let Err(e) = l9_grepr::get_args().and_then(l9_grepr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}