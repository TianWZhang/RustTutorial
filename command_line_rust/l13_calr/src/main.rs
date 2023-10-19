fn main() {
    if let Err(e) = l13_calr::get_args().and_then(l13_calr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
