mod grep;
use std::process;
fn main() {
    if let Err(error) = grep::getting_args().and_then(grep::run) {
        eprintln!("Ooops: {error}");
        process::exit(1);
    }
}
