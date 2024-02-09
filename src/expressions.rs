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
    // /// flashcard
    // Flash,
}
impl Msg {
    /// get value for expression
    pub fn val(&self) -> ColoredString {
        match *self {
            Msg::Quest => format!("{}?", SPACER).bright_yellow().bold(),
            Msg::Knew => format!("{}%", SPACER).bright_green().bold(),
            Msg::KnewIt => "Yes, that's right!".bright_green(),
            Msg::Skip => format!("{}Skipping:", SPACER).bright_magenta(),
            Msg::Revise => {
                format!("{}Going to the ones not guessed correctly...", SPACER).bright_magenta()
            }
            Msg::Typo => format!("{}Corrected: ", SPACER).bright_magenta().italic(),
            Msg::Exit => format!("\n{}Exiting...", SPACER).bright_magenta().italic(),
            Msg::Hint => format!("{}#", SPACER).cyan().bold(),
            Msg::Wrong => format!("{}~", SPACER).bright_red().bold(),
            Msg::WrongIt => "<-is the right answer.".bright_red().italic(),
            // Exp::Flash => format!("{}=", SPACER).bright_cyan().bold(),
        }
    }
}
