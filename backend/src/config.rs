use std::env::VarError;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Environment variable error")]
    EnvVarError(#[from] VarError),
    #[error("Empty variable error")]
    VarEmpty(String),
}

pub type ConfigResult<T> = Result<T, ConfigError>;

#[derive(Clone)]
pub struct Config {
    pub redis_url: String,
}

impl Config {
    pub fn new() -> ConfigResult<Config> {
        dotenv::dotenv().ok();

        Ok(Config {
            redis_url: load_env_str(String::from("REDIS_URL"))?,
        })
    }
}

fn load_env_str(key: String) -> ConfigResult<String> {
    let var = env::var(&key)?;

    if var.is_empty() {
        return Err(ConfigError::VarEmpty(key));
    }

    Ok(var)
}
