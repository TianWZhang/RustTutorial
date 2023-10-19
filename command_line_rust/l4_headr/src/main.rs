fn main() {
    if let Err(e) = l4_headr::get_args().and_then(l4_headr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
