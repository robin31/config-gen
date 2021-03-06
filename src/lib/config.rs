extern crate serde_yaml;

use clap::Error;
use from_file::FromFile;
use from_file::FromFileError;
use options::ConfigError;
use serde_yaml::Value;

#[derive(Deserialize, Debug, Clone)]
pub struct ProgramConfig {
    pub presets: Vec<PresetConfig>,
}

impl FromFile for ProgramConfig {}

impl ProgramConfig {
    pub fn get_opts(&self, name: &str) -> Option<serde_yaml::Value> {
        self.presets
            .iter()
            .find(|p| p.name == name)
            .map(|p| p.options.clone())
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct PresetConfig {
    pub name: String,
    pub options: Value,
}

#[derive(Debug)]
pub enum ProgramStartError {
    ConfigFileOpen,
    ConfigFileRead,
    ConfigParseError(serde_yaml::Error),
    ConfigCliError(ConfigError),
    InvalidArgs(Error),
    FromFile(FromFileError),
}

impl std::fmt::Display for ProgramStartError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProgramStartError::ConfigParseError(e) => write!(f, "could not parse config, {}", e),
            ProgramStartError::ConfigCliError(ConfigError::UrlInvalid(parse_error)) => {
                write!(f, "URL parse error: {}", parse_error)
            }
            ProgramStartError::ConfigCliError(ConfigError::UrlInvalidHost) => {
                write!(f, "{}", ConfigError::UrlInvalidHost)
            }
            ProgramStartError::ConfigCliError(ConfigError::UrlInvalidScheme) => {
                write!(f, "{}", ConfigError::UrlInvalidScheme)
            }
            ProgramStartError::ConfigCliError(ConfigError::UrlInvalidPort) => {
                write!(f, "{}", ConfigError::UrlInvalidPort)
            }
            ProgramStartError::ConfigFileOpen => write!(f, "config file not found"),
            ProgramStartError::ConfigFileRead => write!(f, "config file content could not be read"),
            ProgramStartError::FromFile(e) => write!(f, "{}", e),
            ProgramStartError::InvalidArgs(e) => write!(f, "{}", e),
        }
    }
}
