mod config;
mod image;
mod objects;
mod ray;
mod rt;
mod vec3;

use std::env;
use std::process;

use crate::config::Config;

fn run() -> Result<(), String> {
    let config = Config::new(env::args())?;
    rt::render(config)?;

    Ok(())
}

fn main() {
    process::exit(match run() {
        Ok(_) => 0,
        Err(message) => {
            eprintln!("{}", message);
            1
        }
    });
}
