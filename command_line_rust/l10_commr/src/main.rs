fn main() {
    if let Err(e) = l10_commr::get_args().and_then(l10_commr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
