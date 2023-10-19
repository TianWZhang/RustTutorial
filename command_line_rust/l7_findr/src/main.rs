fn main() {
    if let Err(e) = l7_findr::get_args().and_then(l7_findr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
