use std::fmt;
use std::env;
use std::str::FromStr;

use crate::conf::error::ConfigError;

pub const CONFIG_ENV: &str = "CONFIG_ENV";

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Env {
    Dev,
    Prod,
}

impl Env {
    pub(crate) const ALL: [Env; 2] = [Env::Dev, Env::Prod];

    pub(crate) const VALID: &'static str = "dev, prod";

    pub fn active() -> Result<Env, ConfigError> {
        match env::var(CONFIG_ENV) {
            Ok(s) => s.parse().map_err(|_| ConfigError::BadEnv(s)),
            #[cfg(debug_assertions)]
            _ => Ok(Env::Dev),
            #[cfg(not(debug_assertions))]
            _ => Ok(Env::Prod),
        }
    }

    #[inline]
    pub fn is_dev(self) -> bool {
        self == Env::Dev
    }

    #[inline]
    pub fn is_prod(self) -> bool {
        self == Env::Prod
    }
}


impl FromStr for Env {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dev" | "d" | "development" | "devel" => Ok(Env::Dev),
            "p" | "prod" | "production" => Ok(Env::Prod),
            _ => return Err(()),
        }
    }
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Env::Dev => write!(f, "dev"),
            Env::Prod => write!(f, "prod"),
        }
    }
}