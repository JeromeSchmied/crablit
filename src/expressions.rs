//! # Code containing expressions used in `crablit` regularly.
use colored::Colorize;

/// space before any output
pub const SPACER: &str = "    ";
/// commonly used expressions(text), colored strings
pub enum Msg {
    /// Question(mark)
    Quest(String),
    /// Knew it
    Knew,
    /// skipping the following:
    Skip(String),
    /// goin' to the ones not guessed correctly
    Revise,
    /// Correct the following:
    Typo(String),
    /// Stop executing the program
    Exit,
    /// show hint
    Hint(String),
    /// didn't know mark
    Wrong(String),
    /// flashcard
    Flash,
    /// still this much to go
    Togo(usize, usize),
}
impl Msg {
    /// get value for expression
    pub fn val(&self) -> String {
        match self {
            Self::Quest(s) => format!("{}{} {s}", SPACER, "?".bright_yellow().bold()),
            Self::Knew => format!(
                "{}{} {}",
                SPACER,
                "$".bright_green().bold(),
                "Yes, that's right!\n".bright_green()
            ),
            Self::Skip(s) => format!("{}{} {s}", SPACER.repeat(2), "Skipping:".bright_magenta()),
            Self::Revise => {
                format!(
                    "{}{}",
                    SPACER,
                    "Going to the ones not guessed correctly...".bright_magenta()
                )
            }
            Self::Typo(s) => format!(
                "{}{} {s}",
                SPACER.repeat(2),
                "Corrected:".bright_magenta().italic(),
            ),
            Self::Exit => format!("\n{}{}", SPACER, "Exiting...".bright_magenta().italic()),
            Self::Hint(s) => format!("{}{} {}", SPACER, "#".cyan().bold(), s),
            Self::Wrong(s) => format!(
                "{}{} {s} {}\n\n",
                SPACER,
                "~".bright_red().bold(),
                "<-is the right answer.".bright_red().italic()
            ),
            Self::Flash => format!("{}{} ", SPACER, "=".bright_cyan().bold()),
            Self::Togo(sum, i) => {
                format!(
                    "{}{} {}% {} more to go",
                    SPACER,
                    "!".bold().bright_purple(),
                    i / sum,
                    sum - i
                )
            }
        }
    }
}
