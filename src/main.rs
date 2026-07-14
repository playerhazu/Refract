use std::env;

mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    utils::process_command(&args);
}
