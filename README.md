# Linicon Theme

> [!NOTE]
> This is the mirror of <https://git.sr.ht/~zethra/linicon-theme>  
> this exist because the unstable connection with git.sr.ht(got 504 at 01-24).
>
> **Please let me know if i should delete this**

[![crates.io](https://img.shields.io/crates/v/linicon-theme.svg)](https://crates.io/crates/linicon-theme)
[![docs.rs](https://docs.rs/linicon-theme/badge.svg)](https://docs.rs/linicon-theme)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.48-red)

Get the user's current icon theme on Linux

There isn't a unified standard for getting the current icon theme on Linux.
So linicon-theme attempts to check many places theme information might be
stored. See [the docs](https://docs.rs/linicon-theme) for more details.

## Example

```rust
use linicon_theme::get_icon_theme;

println!("Your current icon theme is: {}", get_icon_theme().unwrap());
```

## Contributing

Please send any and all patches, bugs, and questions to my public inbox
[~zethra/public-inbox@lists.sr.ht](mailto:~zethra/public-inbox@lists.sr.ht)
or submit a ticket to the bug tracker if you feel so inclined
[todo.sr.ht/~zethra/linicon](https://todo.sr.ht/~zethra/linicon).
