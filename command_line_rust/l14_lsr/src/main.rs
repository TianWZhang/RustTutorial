fn main() {
    if let Err(e) = l14_lsr::get_args().and_then(l14_lsr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
