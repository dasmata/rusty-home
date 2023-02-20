use std::io::Write;
use serde::{Deserialize, Serialize};
use confy;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub version: u8,
    pub hubitat_host: String,
    pub hubitat_key: String
}

/// `Config` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            version: 0,
            hubitat_key: "".into(),
            hubitat_host: "".into()
        }
    }
}

pub fn load() -> Result<Config, confy::ConfyError> {
    let config: Config = confy::load("hubitat_cfg", None)?;

    Ok(config)
}

pub fn store(cfg: &Config) -> Result<(), confy::ConfyError> {
    confy::store("hubitat_cfg", None, cfg)
}

pub fn build_config() -> Result<(), confy::ConfyError> {
    let current_config = match load() {
        Ok(c) => c,
        Err(_) => Config::default()
    };
    let mut new_config = Config::default();

    println!("We need to configure your app.");
    print!("Hubitat url: ");
    let flushed = std::io::stdout().flush();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    new_config.hubitat_host = input.trim_end().into();


    let mut input = String::new();
    print!("Hubitat key: ");
    let flushed = std::io::stdout().flush();
    std::io::stdin().read_line(&mut input).unwrap();
    new_config.hubitat_key = input.trim_end().into();
    new_config.version = current_config.version + 1;

    store(&new_config)
}
