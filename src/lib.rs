use atty::Stream;
use std::fmt;

#[cfg(not(target_os = "macos"))]
lazy_static::lazy_static! {
    static ref IS_LANG_UTF8: bool = {
        match std::env::var("LANG") {
            Ok(lang) => lang.to_uppercase().ends_with("UTF-8"),
            _ => false,
        }
    };
}

pub struct Emoji<'a, 'b>(pub &'a str, pub &'b str);

impl<'a, 'b> Emoji<'a, 'b> {
    pub const fn new(emoji: &'a str, fallback: &'b str) -> Self {
        Self(emoji, fallback)
    }
}

impl fmt::Display for Emoji<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if should_display_emoji() {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}", self.1)
        }
    }
}

// Emojis should only get displayed if the current terminal is a tty and the
// platform does support emojis.
fn should_display_emoji() -> bool {
    atty::is(Stream::Stdout) && is_emoji_supported()
}

// The new Windows Terminal does support emojis. Currently, the terminal will
// set the environment variable `WT_SESSION`. This can be used to check if the
// user uses that specific app.
#[cfg(windows)]
fn is_emoji_supported() -> bool {
    std::env::var("WT_SESSION").is_some()
}

// macOS by default has emoji support.
#[cfg(target_os = "macos")]
fn is_emoji_supported() -> bool {
    true
}

// On unix systems the enabled language decides whether emojis are supported or
// not.
#[cfg(not(target_os = "macos"))]
fn is_emoji_supported() -> bool {
    *IS_LANG_UTF8
}
