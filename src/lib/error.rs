use derive_more::From;
use std::{fmt, io};

#[derive(Debug, From)]
pub enum Error {
  IO(io::Error),
  SerdeYaml(serde_yaml::Error),
  HomeDirectoryMissing,
  ConfigurationNotFound,
  ColorSchemesMissing,
  ColorSchemesNotAMapping,
  ColorSchemeNotAvailable,
}
impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::IO(_) => write!(f, "An IO Operation failed"),
      Error::SerdeYaml(_) => write!(f, "Serde Yaml failed to parse the Alacritty config"),
      Error::HomeDirectoryMissing => write!(f, "Could not find the user's home directory"),
      Error::ConfigurationNotFound => write!(f, "Could not find a valid Alacritty config"),
      Error::ColorSchemesMissing => write!(
        f,
        "Could not find any available color schemes, these should be under the \"color_schemes\" key"
      ),
      Error::ColorSchemesNotAMapping => write!(f, "Found the \"color_schemes\" key, but it is not a mapping"),
      Error::ColorSchemeNotAvailable => write!(f, "Color scheme not found\nDoes the `list` subcommand contain this color scheme?"),
    }
  }
}
