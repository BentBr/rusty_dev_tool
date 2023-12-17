mod env;
mod env_functions;
mod error;
mod tests;

use crate::env_functions::read_env_file;

fn main() {
    println!("Hello, world!");

    read_env_file();
}
