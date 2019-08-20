use std::env::Args;

pub struct Config {
    pub file: String,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, String> {
        let program = args.next().unwrap();
        let error_message = format!("Usage: {} <file>", program);

        let file = match args.next() {
            Some(arg) => arg,
            None => return Err(error_message),
        };

        Ok(Config { file })
    }
}
