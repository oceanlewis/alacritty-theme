use termion::color;

mod lib;
use lib::Cli;

fn main() {
  if let Err(error) = Cli::run() {
    eprintln!(
      "{red}Error: {reset}{error}",
      error = error,
      red = color::Fg(color::Red),
      reset = color::Fg(color::Reset)
    );
  }
}

#[cfg(test)]
mod tests {
  use crate::lib::*;

  #[test]
  fn loads_alacritty_config() {
    assert!(AlacrittyConfig::load().is_ok());
  }

  #[test]
  fn changes_color_scheme() -> Result<(), Error> {
    let mut config =
      AlacrittyConfig::load_at_path("./test/alacritty.yml".into())?;

    fn check_successful_change(
      given_theme: &str,
      config: &mut AlacrittyConfig,
    ) {
      assert!(config.change_theme(given_theme).is_ok());
      assert_eq!(
        format!("colors: *{theme}", theme = given_theme),
        config
          .contents
          .split('\n')
          .filter(|line| line.starts_with("colors:"))
          .collect::<String>()
      );
    }

    assert!(config.change_theme("non_existent").is_err());

    check_successful_change("gruvbox_light", &mut config);
    check_successful_change("gruvbox_dark", &mut config);
    check_successful_change("gruvbox_super_dark", &mut config);

    Ok(())
  }
}
