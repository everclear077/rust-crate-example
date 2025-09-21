use self::ConfigError::*;
use std::path::PathBuf;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ConfigError {
    NotFound,
    IoError,
    BadFilePath(PathBuf, &'static str),
    BadEnv(String),
    BadEntry(String, PathBuf),
    BadType(String, &'static str, &'static str, Option<PathBuf>),
    ParseError(String, PathBuf, String, Option<(usize, usize)>),
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            NotFound => "config file not found",
            IoError => "io error",
            BadFilePath(_, _) => "bad file path",
            BadEnv(_) => "bad env",
            BadEntry(_, _) => "bad entry",
            ParseError(_, _, _, _) => "parse error",
            BadType(_, _, _, _) => "bad type",
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            NotFound => write!(f, "config file not found"),
            IoError => write!(f, "io error"),
            BadFilePath(_, _) => write!(f, "bad file path"),
            BadEnv(_) => write!(f, "bad env"),
            BadEntry(_, _) => write!(f, "bad entry"),
            ParseError(_, _, _, _) => write!(f, "parse error"),
            BadType(_, _, _, _) => write!(f, "bad type"),
        }
    }
}