//! Crate config

use crate::error::{Error, Result};
use std::{env, str::FromStr, sync::OnceLock};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHOLE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Config {
    // -- fs
    pub FS_VAR: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            // -- fs
            FS_VAR: get_env("FS_VAR")?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}

fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get_env(name)?;

    val.parse::<T>().map_err(|_| Error::ConfigWrongFormat(name))
}
