use std::fmt;
use terminal_supports_emoji::{supports_emoji, Stream};

/// An emoji with safety fallback.
///
/// The struct wraps an emoji and only renders it on platforms that actually
/// support it. On non-supported platforms the fallback value is being rendered.
///
/// Support is determined by two factors:
///
/// 1) The processes stdout has to be a tty.
/// 2) Platform dependent:
///     - macOS has emoji support by default
///     - Unix systems have support if the active language supports them.
///     - Windows machines running the new Terminal app support emojis.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Emoji<'a>(pub &'a str, pub &'a str);

impl<'a> Emoji<'a> {
    /// Create a new emoji.
    ///
    /// # Arguments
    ///
    /// - `emoji`: The unicode emoji to display.
    /// - `fallback`: The fallback value to use on non-supported platforms.
    pub const fn new(emoji: &'a str, fallback: &'a str) -> Self {
        Self(emoji, fallback)
    }
}

impl fmt::Display for Emoji<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if supports_emoji(Stream::Stdout) {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}", self.1)
        }
    }
}

impl<'a> From<(&'a str, &'a str)> for Emoji<'a> {
    fn from(v: (&'a str, &'a str)) -> Self {
        Emoji(v.0, v.1)
    }
}
