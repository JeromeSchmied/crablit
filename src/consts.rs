//! # Code containing expressions used in `crablit` regularly.
use super::*;

fn data_dir() -> String {
    format!(
        "{}/crablit/",
        dirs::data_dir()
            .expect("Couldn't find data_dir")
            .to_str()
            .unwrap(),
    )
}

/// Returns the existence of path got in state dir
pub fn progress_exists(path: &str) -> bool {
    //     match std::fs::create_dir(self::data_dir() + path) {
    //         Err(err) => {
    //             if err.kind() == std::io::ErrorKind::NotFound {
    //                 false
    //             } else if err.kind() == std::io::ErrorKind::AlreadyExists {
    //                 true
    //             } else {
    //                 todo!("Couldn't determine progress state")
    //             }
    //         }
    //         _ => todo!("Couldn't determine progress state"),
    //     }
    fs::read_to_string(self::data_dir() + path).is_ok()
}

/// Path for statefile of filepath got
pub fn get_state_path(path: &str) -> Result<String, Box<dyn Error>> {
    let pwd = std::env::current_dir()?;
    let pwd = pwd.to_str().expect("Couldn't get working dir.");

    // try to create data_dir, if exists, don't do anything
    if let Err(err) = std::fs::create_dir(self::data_dir()) {
        if err.kind() == std::io::ErrorKind::NotFound {
            std::fs::create_dir_all(self::data_dir())?;
        } else if err.kind() == std::io::ErrorKind::AlreadyExists {
        } else {
            return Err(Box::new(err));
        }
    }

    let current_file_path = &format!("{}/{}", pwd, path).replace('/', "%");

    Ok(format!("{}{}", self::data_dir(), current_file_path))
}

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
