fn main() {
    if let Err(e) = l5_wcr::get_args().and_then(l5_wcr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
