use std::{fs, path};

use crate::lib::Error;

use dirs::home_dir;
use regex::Regex;
use serde_yaml::Value as Yaml;
use termion::{color, style};

#[derive(Debug)]
pub struct AlacrittyConfig {
  path: path::PathBuf,
  pub contents: String,
  available_color_schemes: Vec<String>,
  theme_regex: Regex,
}
impl AlacrittyConfig {
  pub fn load() -> Result<Self, Error> {
    let path = home_dir()
      .ok_or(Error::HomeDirectoryMissing)?
      .join(".config/alacritty/alacritty.yml");

    Self::load_at_path(path)
  }

  pub fn load_at_path(path: path::PathBuf) -> Result<Self, Error> {
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
  pub fn print_themes(&self) {
    println!(
      "{style}Available themes:{reset}",
      style = style::Bold,
      reset = style::Reset
    );
    self.available_color_schemes.iter().for_each(|theme| {
      println!("  - {}", theme);
    });
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

  pub fn print_current_theme(&mut self) {
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

  pub fn save(&self) -> Result<(), Error> {
    fs::write(&self.path, &self.contents)?;
    Ok(())
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
