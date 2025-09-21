use std::{path::{PathBuf, Path}};
use std::io::{Read};
use crate::env::Env;
use crate::conf::error::ConfigError;


pub type Result<T> = ::std::result::Result<T, ConfigError>;


#[derive(Debug)]
pub struct Database {
    pub adapter: String,
    pub name: String,
    pub pool: u16,
}

#[derive(Debug)]
pub struct Config {
    pub env: Env,
    pub address: String,
    pub port: u16,
    pub workers: Option<u16>,
    pub database: Option<Database>,
    pub(crate) file_path: Option<PathBuf>,
    pub(crate) root_path: Option<PathBuf>,
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address 
            && self.port == other.port 
            && self.workers == other.workers
    }
}

impl Config {
    pub fn new(env: Env) -> Self {
        Self::default(env)
    }

    pub(crate) fn default(env: Env) -> Self {
        let default_workers = (num_cpus::get() * 2) as u16;
        let default_config = Config {
            env: env,
            address: "localhost".to_string(),
            port: 8080,
            workers: Some(default_workers),
            database: None,
            file_path: None,
            root_path: None,
        };
        match env {
            Env::Dev  => {
                Config {
                    env: Env::Dev,
                    ..default_config
                }
            }
            Env::Prod => {
                Config {
                    env: Env::Prod,
                    ..default_config
                }
            }
        }
    }

    pub fn set_root<P: AsRef<Path>>(&mut self, path: P) {
        self.root_path = Some(path.as_ref().into());
    }

    pub fn default_from<P>(env: Env, path: P) -> Result<Self>
    where
        P: AsRef<Path>
    {
        let mut config = Config::default(env);
        let file_path = path.as_ref().to_path_buf();
        if let Some(parent) = file_path.parent() {
            config.set_root(parent);
        } else {
            return Err(ConfigError::BadFilePath(file_path.clone(), "no parent directory"));
        }

        config.file_path = Some(file_path);
        Ok(config)
    }
}