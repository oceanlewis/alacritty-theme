use std::{fs, path::PathBuf};

use crate::lib::Error;

use dirs::home_dir;
use regex::Regex;
use serde_yaml::Value as Yaml;
use termion::{color, style};

#[cfg(not(windows))]
use xdg;

#[derive(Debug)]
pub struct AlacrittyConfig {
  path: PathBuf,
  pub contents: String,
  available_color_schemes: Vec<String>,
  theme_regex: Regex,
}
impl AlacrittyConfig {
  /// This function loads an `AlacrittyConfig` from either an explicit
  /// parameter, or from a list of possible configuration file locations as
  /// specified [here](https://github.com/jwilm/alacritty#configuration).
  pub fn load(given_path: Option<PathBuf>) -> Result<Self, Error> {
    if let Some(final_path) = given_path.or_else(alacritty::installed_config) {
      Self::load_at_path(final_path)
    } else {
      Err(Error::ConfigurationNotFound)
    }
  }

  pub fn save(&self) -> Result<(), Error> {
    fs::write(&self.path, &self.contents)?;
    Ok(())
  }

  pub fn change_theme(&mut self, new_theme: &str) -> Result<(), Error> {
    if self
      .available_color_schemes
      .contains(&new_theme.to_string())
    {
      self.contents = self
        .theme_regex
        .replace_all(&self.contents, format!("colors: *{}", new_theme).as_str())
        .to_string();
      Ok(())
    } else {
      Err(Error::ColorSchemeNotAvailable)
    }
  }

  pub fn print_current_theme(&self) {
    println!(
      "{style}Current theme:{reset}\n{current_theme}",
      style = style::Bold,
      reset = style::Reset,
      current_theme = self
        .theme_regex
        .captures_iter(&self.contents)
        .filter_map(|c| c.get(1))
        .map(|theme| format!("  - {}", theme.as_str()))
        .collect::<String>()
    );
  }

  pub fn print_all_themes(&self) {
    println!(
      "{style}Available themes:{reset}",
      style = style::Bold,
      reset = style::Reset
    );
    self.available_color_schemes.iter().for_each(|theme| {
      println!("  - {}", theme);
    });
  }

  fn load_at_path(path: PathBuf) -> Result<Self, Error> {
    let contents = fs::read_to_string(&path)?;
    let available_color_schemes =
      Self::get_color_schemes(&serde_yaml::from_str(&contents)?)?;
    let theme_regex =
      Regex::new(r#"(?m)^colors: \*([[:word:]]*)[[:blank]]*"#).unwrap();

    Ok(AlacrittyConfig {
      path,
      contents,
      available_color_schemes,
      theme_regex,
    })
  }

  fn get_color_schemes(alacritty_config: &Yaml) -> Result<Vec<String>, Error> {
    match alacritty_config
      .get("color_schemes")
      .ok_or(Error::ColorSchemesMissing)?
    {
      Yaml::Mapping(mapping) => Ok(
        mapping
          .iter()
          .filter_map(|(key, _)| {
            if let Yaml::String(key) = key {
              Some(key.to_owned())
            } else {
              eprintln!(
                "{red}Error: {reset}{error}",
                error = format!("Encountered bad color scheme: {:?}", key),
                red = color::Fg(color::Red),
                reset = color::Fg(color::Reset)
              );
              None
            }
          })
          .collect(),
      ),
      Yaml::Null => Err(Error::ColorSchemesMissing),
      _ => Err(Error::ColorSchemesNotAMapping),
    }
  }
}

/// The following code has been taken from the Alacritty project and should
/// be considered licensed separately from this project.
///
/// See https://github.com/jwilm/alacritty/blob/master/alacritty/src/config
pub(self) mod alacritty {
  use super::*;

  #[cfg(windows)]
  pub fn installed_config() -> Option<PathBuf> {
    dirs::config_dir()
      .map(|path| path.join("alacritty\\alacritty.yml"))
      .filter(|new| new.exists())
  }

  /// Get the location of the first found default config file paths
  /// according to the following order:
  ///
  /// 1. $XDG_CONFIG_HOME/alacritty/alacritty.yml
  /// 2. $XDG_CONFIG_HOME/alacritty.yml
  /// 3. $HOME/.config/alacritty/alacritty.yml
  /// 4. $HOME/.alacritty.yml
  #[cfg(not(windows))]
  pub fn installed_config() -> Option<PathBuf> {
    // Try using XDG location by default
    xdg::BaseDirectories::with_prefix("alacritty")
      .ok()
      .and_then(|xdg| xdg.find_config_file("alacritty.yml"))
      .or_else(|| {
        xdg::BaseDirectories::new()
          .ok()
          .and_then(|fallback| fallback.find_config_file("alacritty.yml"))
      })
      .or_else(|| {
        if let Some(home) = home_dir() {
          // Fallback path: $HOME/.config/alacritty/alacritty.yml
          let fallback =
            PathBuf::from(&home).join(".config/alacritty/alacritty.yml");
          if fallback.exists() {
            return Some(fallback);
          }
          // Fallback path: $HOME/.alacritty.yml
          let fallback = PathBuf::from(&home).join(".alacritty.yml");
          if fallback.exists() {
            return Some(fallback);
          }
        }
        None
      })
  }
}
