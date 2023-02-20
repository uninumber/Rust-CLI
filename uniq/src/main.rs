mod uniq;
use std::process;

fn main() {
    if let Err(_) = uniq::get_args().and_then(uniq::run) {
        eprintln!("something went wrong");
        process::exit(1);
    }
}
