use crate::lib::{AlacrittyConfig, Error};

use std::path::PathBuf;
use structopt::{clap::AppSettings, StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(
  name = "Alacritty Themes",
  about = "List and swap your Alacritty themes.",
  setting(AppSettings::ColoredHelp)
)]
pub enum Cli {
  #[structopt(
    name = "list",
    about = "Print a list of your available Alacritty themes",
    setting(AppSettings::ColoredHelp)
  )]
  PrintAllThemes {
    /// Specifies the path of the Alacritty config
    #[structopt(short = "c", long)]
    config_file: Option<PathBuf>,
  },

  #[structopt(
    name = "current",
    about = "Print the current Alacritty theme",
    setting(AppSettings::ColoredHelp)
  )]
  PrintCurrentTheme {
    /// Specifies the path of the Alacritty config
    #[structopt(short = "c", long)]
    config_file: Option<PathBuf>,
  },

  #[structopt(
    name = "change",
    about = "Change the current Alacritty theme to the given theme",
    setting(AppSettings::ColoredHelp)
  )]
  ChangeTheme {
    theme: String,
    /// Specifies the path of the Alacritty config
    #[structopt(short = "c", long)]
    config_file: Option<PathBuf>,
  },
}
impl Cli {
  pub fn run() -> Result<(), Error> {
    match Cli::from_args() {
      Cli::PrintAllThemes { config_file } => {
        let alacritty_config = AlacrittyConfig::load(config_file)?;
        alacritty_config.print_all_themes();
      }

      Cli::ChangeTheme { theme, config_file } => {
        let mut alacritty_config = AlacrittyConfig::load(config_file)?;
        alacritty_config.change_theme(&theme)?;
        alacritty_config.save()?;
      }

      Cli::PrintCurrentTheme { config_file } => {
        let alacritty_config = AlacrittyConfig::load(config_file)?;
        alacritty_config.print_current_theme();
      }
    };

    Ok(())
  }
}
