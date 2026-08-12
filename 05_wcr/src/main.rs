fn main() {
    if let Err(e) = wcr::get_args().and_then(wcr::run) {
        eprint!("{}", e);
        std::process::exit(1)
    }
}
