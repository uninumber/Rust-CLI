use std::process;
fn main() {
    if let Err(e) = head::get_args().and_then(head::run) {
        eprintln!("Something went wrong : {e}");
        process::exit(1);
    }
}
