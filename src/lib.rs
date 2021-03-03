use crossterm::tty::IsTty;
use std::fmt;
use std::io::stdout;

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
        if stdout().is_tty() && is_emoji_supported() {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}", self.1)
        }
    }
}

#[cfg(windows)]
fn is_emoji_supported() -> bool {
    std::env::var("WT_SESSION").is_some()
}

#[cfg(target_os = "macos")]
fn is_emoji_supported() -> bool {
    true
}

#[cfg(not(target_os = "macos"))]
fn is_emoji_supported() -> bool {
    true
}
