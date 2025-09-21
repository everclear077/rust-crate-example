use std::{env, fs, path::{PathBuf, Path}};
use std::collections::HashMap;
use std::io::{Read};
use std::fs::File;
use crate::env::Env;
use crate::conf::error::ConfigError;
use toml::Value;
use crate::conf::config::Config;
use crate::conf::config::Result;

const CONFIG_FILENAME: &str = "config/application.toml";

#[doc(hidden)]
#[derive(Debug, PartialEq)]
pub struct GlobalConfig {
    pub env: Env,
    config: HashMap<Env, Config>,
}


impl GlobalConfig {
    pub fn read_config() -> Result<GlobalConfig> {
        let f = GlobalConfig::find()?;

        let mut handle = File::open(&f).map_err(|_| ConfigError::IoError)?;

        let mut content = String::new();
        handle.read_to_string(&mut content).map_err(|_| ConfigError::IoError)?;

        GlobalConfig::parse(content, &f)
    }

    fn find() -> Result<PathBuf> {
        let pwd = env::current_dir().map_err(|_| ConfigError::NotFound)?;
        let mut current = pwd.as_path();

        loop {
            let config_path = current.join(CONFIG_FILENAME);
            if fs::metadata(&config_path).is_ok() {
                return Ok(config_path);
            }
            match current.parent() {
                Some(parent) => current = parent,
                None => break,
            }
        }

        Err(ConfigError::NotFound)
    }

    fn get_mut(&mut self, env: Env) -> &mut Config {
        match self.config.get_mut(&env) {
            Some(config) => config,
            None => panic!("set(): {} config is missing", env),
        }
    }

    pub fn active_default_from(filename: Option<&Path>) -> Result<GlobalConfig> {
        let mut defaults = HashMap::new();
        if let Some(path) = filename {
            defaults.insert(Env::Dev, Config::default_from(Env::Dev, &path)?);
            defaults.insert(Env::Prod, Config::default_from(Env::Prod, &path)?);
        } else {
            defaults.insert(Env::Dev, Config::default(Env::Dev));
            defaults.insert(Env::Prod, Config::default(Env::Prod));
        }

        let config =  GlobalConfig {
            env: Env::active()?,
            config: defaults,
        };

        Ok(config)
    }

    fn parse<P: AsRef<Path>>(src: String, filename: P) -> Result<GlobalConfig> {
        let path = filename.as_ref().to_path_buf();
        let table = match src.parse::<Value>() {
            Ok(Value::Table(table)) => table,
            Ok(value) => {
                let err = format!("expected a table, but got a {}", value.type_str());
                return Err(ConfigError::ParseError(src, path, err, None));
            }
            Err(e) => return Err(ConfigError::ParseError(src, path, e.to_string(), None)),
        };

        let config = GlobalConfig::active_default_from(Some(filename.as_ref()))?;

        for (entry, value) in table {
            match value.as_table() {
                Some(table) => table,
                None => return Err(ConfigError::BadType(
                    entry, "a table", value.type_str(), Some(path.clone())
                ))
            };
        }
        Ok(config)
    }
}
