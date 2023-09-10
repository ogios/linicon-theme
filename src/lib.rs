//! Get the user's current icon theme on Linux
//!
//! There isn't a unified standard for getting the current icon theme on Linux.
//! So linicon-theme attempts to check many places theme information might be
//! stored.  See [`get_icon_theme`](fn.get_icon_theme.html) for more details.
//!
//! ## Example
//! ```
//! use linicon_theme::get_icon_theme;
//!
//! println!("Your current icon theme is: {}", get_icon_theme().unwrap());
//! ```
#![forbid(unsafe_code)]

use freedesktop_entry_parser::parse_entry;
use ini::Ini;
use std::{env, ffi::OsString, process::Command, str};

/// Get the user's current icon theme
///
/// There isn't a unified standard for getting the current icon theme on Linux.
/// So linicon-theme attempts to check many places theme information might be
/// stored.  The following places are checked in order.
///
/// - `$XDG_CONFIG_HOME/kdeglobals` -> Icons -> Theme
/// - Output of `gsettings get org.gnome.desktop.interface icon-theme`
/// - `$XDG_CONFIG_HOME/gtk-3.0/settings.ini` -> Settings -> gtk-icon-theme-name
/// - `$HOME/.gtkrc-2.0` -> gtk-icon-theme-name
/// - `$XDG_CONFIG_HOME/theme.conf` -> Settings -> icon-theme-name
///
/// Returns `None` if the theme can't be found for some reason.
pub fn get_icon_theme() -> Option<String> {
    get_icon_theme_order(&[
        Check::KDEGlobals,
        Check::GSettings,
        Check::GTK3,
        Check::GTK2,
        Check::ThemeConf,
    ])
}

/// The same as [`get_icon_theme`](fn.get_icon_theme.html) except
/// you can chose which icon theme locations are checked and in
/// what order.
pub fn get_icon_theme_order(order: &[Check]) -> Option<String> {
    let home_path = env::var_os("HOME")?;
    for check in order {
        match check {
            Check::KDEGlobals => {
                if let Some(s) = kde(home_path.clone()) {
                    return Some(s);
                }
            }
            Check::GSettings => {
                if let Some(s) = gsettings() {
                    return Some(s);
                }
            }
            Check::GTK3 => {
                if let Some(s) = gtk3(home_path.clone()) {
                    return Some(s);
                }
            }
            Check::GTK2 => {
                if let Some(s) = gtk2(home_path.clone()) {
                    return Some(s);
                }
            }
            Check::ThemeConf => {
                if let Some(s) = theme_conf(home_path.clone()) {
                    return Some(s);
                }
            }
        }
    }
    None
}

/// Select which theme store locations to check
pub enum Check {
    /// `$XDG_CONFIG_HOME/kdeglobals` -> Icons -> Theme
    KDEGlobals,
    /// Output of `gsettings get org.gnome.desktop.interface icon-theme`
    GSettings,
    /// `$XDG_CONFIG_HOME/gtk-3.0/settings.ini` -> Settings -> gtk-icon-theme-name
    GTK3,
    /// `$HOME/.gtkrc-2.0` -> gtk-icon-theme-name
    GTK2,
    /// `$XDG_CONFIG_HOME/theme.conf` -> Settings -> icon-theme-name
    ThemeConf,
}

fn kde(home_path: OsString) -> Option<String> {
    let mut path = conf_dir(home_path)?;
    path.push("/kdeglobals");
    let file = parse_entry(path).ok()?;
    file.section("Icons").attr("Theme").map(|s| s.to_owned())
}

fn gtk2(mut home_path: OsString) -> Option<String> {
    home_path.push("/.gtkrc-2.0");
    let file = Ini::load_from_file(home_path).ok()?;
    file.get_from::<String>(None, "gtk-icon-theme-name")
        .map(|s| s.to_owned())
}

fn gtk3(home_path: OsString) -> Option<String> {
    let mut path = conf_dir(home_path)?;
    path.push("/gtk-3.0/settings.ini");
    let file = Ini::load_from_file(path).ok()?;
    file.get_from(Some("Settings"), "gtk-icon-theme-name")
        .map(|s| s.to_owned())
}

fn gsettings() -> Option<String> {
    let output = Command::new("gsettings")
        .args(&["get", "org.gnome.desktop.interface", "icon-theme"])
        .output()
        .ok()?
        .stdout;
    let s = str::from_utf8(&output).ok()?;
    // Remove new line and quotes if present
    let s = match s.strip_suffix('\n') {
        Some(s) => s,
        None => s,
    };
    let s = match s.strip_prefix('\'') {
        Some(s) => s,
        None => s,
    };
    let s = match s.strip_suffix('\'') {
        Some(s) => s,
        None => s,
    };
    Some(s.to_owned())
}

fn theme_conf(home_path: OsString) -> Option<String> {
    let mut path = conf_dir(home_path)?;
    path.push("/theme.conf");
    let file = Ini::load_from_file(path).ok()?;
    file.get_from(Some("Settings"), "icon-theme-name")
        .map(|s| s.to_owned())
}

fn conf_dir(mut home_path: OsString) -> Option<OsString> {
    match env::var_os("XDG_CONFIG_HOME") {
        Some(s) => Some(s),
        None => {
            home_path.push("/.config");
            Some(home_path)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kde_test() {
        let home_path = env::var_os("HOME").unwrap();
        let target_name = env::var("TEST_THEME").unwrap();
        assert_eq!(kde(home_path), Some(target_name));
    }

    #[test]
    fn gtk3_test() {
        let home_path = env::var_os("HOME").unwrap();
        let target_name = env::var("TEST_THEME").unwrap();
        assert_eq!(gtk3(home_path), Some(target_name));
    }

    #[test]
    fn gtk2_test() {
        let home_path = env::var_os("HOME").unwrap();
        let target_name = env::var("TEST_THEME").unwrap();
        assert_eq!(gtk2(home_path), Some(target_name));
    }

    #[test]
    fn gsettings_test() {
        let target_name = env::var("TEST_THEME").unwrap();
        assert_eq!(gsettings(), Some(target_name));
    }

    #[test]
    fn theme_conf_test() {
        let home_path = env::var_os("HOME").unwrap();
        let target_name = env::var("TEST_THEME").unwrap();
        assert_eq!(theme_conf(home_path), Some(target_name));
    }
}
