# Linicon Theme

Get the user's current icon theme on Linux

There isn't a unified standard for getting the current icon theme on Linux.
So linicon-theme attempts to check many places theme information might be
stored.  See [the docs](https://docs.rs/linicon-theme) for more details.

## Example

```rust
use linicon_theme::get_icon_theme;

println!("Your current icon theme is: {}", get_icon_theme().unwrap());
```
