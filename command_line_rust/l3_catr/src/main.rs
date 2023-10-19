fn main() {
    if let Err(e) = l3_catr::get_args().and_then(l3_catr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
