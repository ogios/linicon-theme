//! Get the user's current icon theme on Linux
use freedesktop_entry_parser::parse_entry;
use ini::Ini;
use std::{env, ffi::OsString, process::Command, str};

/// Get the user's current icon theme
///
///
pub fn get_icon_theme() -> Option<String> {
    get_icon_theme_order(&[
        Check::KDEGlobals,
        Check::GSettings,
        Check::GTK3,
        Check::GTK2,
        Check::ThemeConf,
    ])
}

pub fn get_icon_theme_order(order: &[Check]) -> Option<String> {
    let home_path = env::var_os("HOME")?;
    for check in order {
        match check {
            Check::KDEGlobals => match kde(home_path.clone()) {
                Some(s) => return Some(s),
                None => (),
            },
            Check::GSettings => match gsettings() {
                Some(s) => return Some(s),
                None => (),
            },
            Check::GTK3 => match gtk3(home_path.clone()) {
                Some(s) => return Some(s),
                None => (),
            },
            Check::GTK2 => match gtk2(home_path.clone()) {
                Some(s) => return Some(s),
                None => (),
            },
            Check::ThemeConf => match theme_conf(home_path.clone()) {
                Some(s) => return Some(s),
                None => (),
            },
        }
    }
    None
}

pub enum Check {
    KDEGlobals,
    GSettings,
    GTK3,
    GTK2,
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
