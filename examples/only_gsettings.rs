use linicon_theme::{get_icon_theme_order, Check};

fn main() {
    println!(
        "Your icon theme according to gsettings is: {}",
        get_icon_theme_order(&[Check::GSettings]).unwrap()
    );
}
