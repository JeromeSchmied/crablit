//! # Code containing expressions used in `crablit` regularly.

use owo_colors::OwoColorize;
use std::fmt;

/// space before any output
pub const SPCR: &str = "    ";

/// Knew it text.
pub fn knew_msg() -> String {
    format!(
        "{SPCR}{} {}",
        "$".bright_green().bold(),
        "Yes, that's right!\n".bright_green()
    )
}
/// Revising text.
pub fn revise_msg() -> String {
    format!(
        "{SPCR}{}",
        "Going to the ones not guessed correctly...".bright_magenta()
    )
}
/// Typo text.
pub fn typo_msg(s: &str) -> String {
    format!(
        "{}{} {s}",
        SPCR.repeat(2),
        "Corrected:".bright_magenta().italic(),
    )
}
/// Exiting text.
pub fn exit_msg() -> String {
    format!("\n{SPCR}{}", "Exiting...".bright_magenta().italic())
}
/// To go text.
pub fn togo_msg(sum: usize, i: usize) -> String {
    format!(
        "{}{} at {:.1}{}, {} more to go",
        SPCR.repeat(2),
        "!".bold().bright_purple(),
        (i as f32 / sum as f32 * 100.),
        "%".bold().bright_purple(),
        (sum + 1 - i).to_string().italic()
    )
}

#[derive(Debug, Clone, PartialEq)]
/// Level Of Knowledge
pub enum Lok {
    /// has not seen it yet
    Nothing,
    /// got once right
    Something,
    /// got twice rigth
    Almost,
    /// got 3 times right: done
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
    /// Increment this [`Lok`].
    ///
    /// # Examples
    ///
    /// ```
    /// use crablit::Lok;
    ///
    /// let mut lok = Lok::default();
    /// lok.incr();
    /// assert_eq!(lok, Lok::Something);
    /// ```
    pub fn incr(&mut self) {
        *self = match *self {
            Self::Nothing => Self::Something,
            Self::Something => Self::Almost,
            Self::Almost | Self::Done => Self::Done,
        }
    }
    /// Decrement this [`Lok`].
    ///
    /// # Examples
    ///
    /// ```
    /// use crablit::utils::Lok;
    ///
    /// let mut lok = Lok::Almost;
    /// lok.decr();
    /// assert_eq!(lok, Lok::Something);
    /// ```
    pub fn decr(&mut self) {
        *self = match *self {
            Self::Nothing | Self::Something => Self::Nothing,
            Self::Almost => Self::Something,
            Self::Done => Self::Almost,
        }
    }
}
impl Default for Lok {
    fn default() -> Self {
        Self::Nothing
    }
}
impl fmt::Display for Lok {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Nothing => "Nothing",
                Self::Something => "Something",
                Self::Almost => "Almost",
                Self::Done => "Done",
            }
        )?;
        Ok(())
    }
}
