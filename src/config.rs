use config;

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(::config::Environment::default())
            .build()?
            .try_deserialize()
    }
}
