//! # Code containing expressions used in `crablit` regularly.
use owo_colors::OwoColorize;

/// space before any output
pub const SPACER: &str = "    ";

pub fn knew() -> String {
    format!(
        "{SPACER}{} {}",
        "$".bright_green().bold(),
        "Yes, that's right!\n".bright_green()
    )
}
pub fn revise() -> String {
    format!(
        "{SPACER}{}",
        "Going to the ones not guessed correctly...".bright_magenta()
    )
}

pub fn typo(s: &str) -> String {
    format!(
        "{}{} {s}",
        SPACER.repeat(2),
        "Corrected:".bright_magenta().italic(),
    )
}
pub fn exit() -> String {
    format!("\n{SPACER}{}", "Exiting...".bright_magenta().italic())
}
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

#[derive(Debug, PartialEq)]
/// Type of Deck
pub enum Mode {
    /// Basic term-definition
    Cards,
    /// Convert from `Verbs` to `Cards`. term as term, infinitive as definition.
    VerbsToCards,
}
impl Mode {
    /// Creates new instance of `Self`
    ///
    /// # Usage
    /// ```
    /// use crablit::Mode;
    ///
    /// let mode = Mode::from("cards");
    ///
    /// assert_eq!(mode, Mode::Cards);
    /// ```
    /// # Panics
    ///
    /// if mode is neither cards, or conv
    pub fn from(mode: &str) -> Self {
        let s = &mode.to_lowercase();
        if s == "cards" || s == "card" {
            Self::Cards
        } else if s == "conv" || s == "convert" || s == "verb_conv" || s == "verbs2cards" {
            Self::VerbsToCards
        } else {
            panic!("Couldn't determine type of deck: it wasn't 'cards', 'verbs' or 'verbs2cards'!");
        }
    }

    /// Creates conviniently displayable String
    /// # usage
    /// ```
    /// use crablit::Mode;
    ///
    /// let mode = Mode::from("convert");
    ///
    /// assert_eq!("convert", mode.disp())
    /// ```
    pub fn disp(&self) -> String {
        match self {
            Mode::Cards => "cards".into(),
            Mode::VerbsToCards => "convert".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// `LevelOfKnowledge`Rendering
pub enum Lok {
    Nothing,
    Something,
    Almost,
    Done,
}
impl Lok {
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
