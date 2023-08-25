#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let case_insensitive = std::env::var("CASE_INSENSITIVE").is_ok();
        Ok(Config {
            query,
            file_path,
            case_insensitive,
        })
    }
}
