//! # Code containing expressions used in `crablit` regularly.
use owo_colors::OwoColorize;

/// space before any output
pub const SPACER: &str = "    ";

/// Knew it text.
pub fn knew() -> String {
    format!(
        "{SPACER}{} {}",
        "$".bright_green().bold(),
        "Yes, that's right!\n".bright_green()
    )
}
/// Revising text.
pub fn revise() -> String {
    format!(
        "{SPACER}{}",
        "Going to the ones not guessed correctly...".bright_magenta()
    )
}
/// Typo text.
pub fn typo(s: &str) -> String {
    format!(
        "{}{} {s}",
        SPACER.repeat(2),
        "Corrected:".bright_magenta().italic(),
    )
}
/// Exiting text.
pub fn exit() -> String {
    format!("\n{SPACER}{}", "Exiting...".bright_magenta().italic())
}
/// To go text.
pub fn togo(sum: usize, i: usize) -> String {
    format!(
        "{}{} at {:.1}{}, {} more to go",
        SPACER.repeat(2),
        "!".bold().bright_purple(),
        (i as f32 / sum as f32 * 100.),
        "%".bold().bright_purple(),
        (sum + 1 - i).to_string().italic()
    )
}

#[derive(Debug, Clone, PartialEq)]
/// Level Of Knowledge
pub enum Lok {
    Nothing,
    Something,
    Almost,
    Done,
}
impl Lok {
    /// Creates a new [`Lok`].
    ///
    /// # Examples
    ///
    /// ```
    /// use crablit::Lok;
    ///
    /// assert_eq!(Lok::new("1"), Lok::Something);
    /// ```
    pub fn new(s: &str) -> Self {
        let s = s.trim();
        if s == "Nothing" || s == "0" {
            Self::Nothing
        } else if s == "Something" || s == "1" {
            Self::Something
        } else if s == "Almost" || s == "2" {
            Self::Almost
        } else if s == "Done" || s == "3" {
            Self::Done
        } else {
            Self::default()
        }
    }
    pub fn incr(&mut self) {
        *self = match *self {
            Self::Nothing => Self::Something,
            Self::Something => Self::Almost,
            Self::Almost | Self::Done => Self::Done,
        }
    }
    pub fn decr(&mut self) {
        *self = match *self {
            Self::Nothing | Self::Something => Self::Nothing,
            Self::Almost => Self::Something,
            Self::Done => Self::Almost,
        }
    }
    pub fn display(&self) -> String {
        match *self {
            Self::Nothing => String::from("Nothing"),
            Self::Something => String::from("Something"),
            Self::Almost => String::from("Almost"),
            Self::Done => String::from("Done"),
        }
    }
}
impl Default for Lok {
    fn default() -> Self {
        Self::Nothing
    }
}
