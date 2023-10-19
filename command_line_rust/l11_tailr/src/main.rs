fn main() {
    if let Err(e) = l11_tailr::get_args().and_then(l11_tailr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
