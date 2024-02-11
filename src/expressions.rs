//! # Code containing expressions used in `crablit` regularly.
use colored::{ColoredString, Colorize};

/// space before any output
pub const SPACER: &str = "    ";
/// commonly used expressions(text), colored strings
pub(crate) enum Msg {
    /// Question(mark)
    Quest,
    /// Knew it mark
    Knew,
    /// Knew it text
    KnewIt,
    /// skipping the following:
    Skip,
    /// goin' to the ones not guessed correctly
    Revise,
    /// Correct the following:
    Typo,
    /// Stop executing the program
    Exit,
    /// show hint
    Hint,
    /// didn't know mark
    Wrong,
    /// didn't know text
    WrongIt,
    /// flashcard
    Flash,
}
impl Msg {
    /// get value for expression
    pub fn val(&self) -> ColoredString {
        match *self {
            Self::Quest => format!("{}? ", SPACER).bright_yellow().bold(),
            Self::Knew => format!("{}% ", SPACER).bright_green().bold(),
            Self::KnewIt => "Yes, that's right!\n".bright_green(),
            Self::Skip => format!("{}Skipping: ", SPACER.repeat(2)).bright_magenta(),
            Self::Revise => {
                format!("{}Going to the ones not guessed correctly...", SPACER).bright_magenta()
            }
            Self::Typo => format!("{}Corrected: ", SPACER.repeat(2))
                .bright_magenta()
                .italic(),
            Self::Exit => format!("\n{}Exiting...", SPACER).bright_magenta().italic(),
            Self::Hint => format!("{}# ", SPACER).cyan().bold(),
            Self::Wrong => format!("{}~ ", SPACER).bright_red().bold(),
            Self::WrongIt => " <-is the right answer.\n".bright_red().italic(),
            Self::Flash => format!("{}=", SPACER).bright_cyan().bold(),
        }
    }
}
