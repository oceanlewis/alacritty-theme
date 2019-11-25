use crate::lib::{AlacrittyConfig, Error};

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
  ListThemes,

  #[structopt(
    name = "current",
    about = "Print the current Alacritty theme",
    setting(AppSettings::ColoredHelp)
  )]
  PrintCurrentTheme,

  #[structopt(
    name = "change",
    about = "Change the current Alacritty theme to the given theme",
    setting(AppSettings::ColoredHelp)
  )]
  ChangeTheme { theme: String },
}
impl Cli {
  pub fn run() -> Result<(), Error> {
    let mut config = AlacrittyConfig::load()?;

    match Cli::from_args() {
      Cli::ListThemes => config.print_themes(),
      Cli::ChangeTheme { theme } => {
        config.change_theme(&theme)?;
        config.save()?;
      }
      Cli::PrintCurrentTheme => config.print_current_theme(),
    };
    Ok(())
  }
}
