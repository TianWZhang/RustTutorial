fn main() {
    if let Err(e) = l6_uniqr::get_args().and_then(l6_uniqr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
