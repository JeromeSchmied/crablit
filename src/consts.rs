//! # Code containing expressions used in `crablit` regularly.
use super::*;

/// space before any output
pub const SPACER: &str = "    ";
/// commonly used expressions(text), colored strings
pub(crate) enum Exp {
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
impl Exp {
    /// get value for expression
    pub fn val(&self) -> ColoredString {
        match *self {
            Exp::Quest => format!("{}?", SPACER).bright_yellow().bold(),
            Exp::Knew => format!("{}%", SPACER).bright_green().bold(),
            Exp::KnewIt => "That's about it!".bright_green(),
            Exp::Skip => format!("{}Skipping:", SPACER).bright_magenta(),
            Exp::Revise => {
                format!("{}Going to the ones not guessed correctly...", SPACER).bright_magenta()
            }
            Exp::Typo => format!("{}Corrected: ", SPACER).bright_magenta().italic(),
            Exp::Exit => format!("\n{}Exiting...", SPACER).bright_magenta().italic(),
            Exp::Hint => format!("{}#", SPACER).cyan().bold(),
            Exp::Wrong => format!("{}~", SPACER).bright_red().bold(),
            Exp::WrongIt => "<-is the right answer.".bright_red().italic(),
            // Exp::Flash => format!("{}=", SPACER).bright_cyan().bold(),
        }
    }
}
