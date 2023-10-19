fn main() {
    if let Err(e) = l8_cutr::get_args().and_then(l8_cutr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
