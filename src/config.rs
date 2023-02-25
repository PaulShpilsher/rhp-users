use envy;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    #[serde(default = "default_port")]
    pub port: u16
}

impl Configuration {
    pub fn new() -> Configuration {
        let c = envy::from_env::<Configuration>().expect("Please provide PORT env var");
        c
    }
}

fn default_port() -> u16 {
    8181
}
