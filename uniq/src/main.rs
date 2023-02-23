mod uniq;
use std::process;

fn main() {
    if let Err(error) = uniq::get_args().and_then(uniq::run) {
        eprintln!("something went wrong : {error}");
        process::exit(1);
    }
}
