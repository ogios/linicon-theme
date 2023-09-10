use linicon_theme::get_icon_theme;

fn main() {
    println!("Your current icon theme is: {}", get_icon_theme().unwrap());
}
