//! # In this module, you can find code that helps in collecting cli arguments and determining properties of a file containing vocab data.
use crate::*;
// use assert_cmd::Command;
use clap::Parser;
use std::{collections::HashMap, error::Error, path::PathBuf};

#[derive(Parser, Debug, PartialEq)]
#[command(version, about, author, long_about = None)]
pub struct Config {
    /// Path of the file to learn
    #[arg(required = true)]
    file_path: PathBuf,

    /// Swap terms and definitions of cards
    #[arg(short = 's', long, default_value_t = false)]
    pub swap: bool,

    /// Sometimes ask the term, sometimes definition of cards
    #[arg(short, long, default_value_t = false)]
    pub ask_both: bool,

    /// Convert valid verbs to cards
    #[arg(long, default_value_t = false)]
    pub convert: bool,

    /// Delimiter used in file to seperate terms and definitions
    #[arg(short, long, default_value = "None")]
    delim: String,

    /// Don't shuffle card order
    #[arg(long, default_value_t = false)]
    pub no_shuffle: bool,

    /// Don't load previous state
    #[arg(long, default_value_t = false)]
    pub no_state: bool,

    /// Only check file syntax don't actually start learning deck
    #[arg(long = "check", default_value_t = false)]
    pub only_check: bool,
}

impl Config {
    /// Fixing properties by opening file that contains vocab data.
    ///
    /// # Errors
    ///
    /// - `fs::create()`
    /// - `get_prog_path()`
    /// - `writeln!()`
    /// - `fs::read_to_string()`
    /// - `get_delim()`
    ///
    /// # Panics
    ///
    /// `delim` is empty
    pub fn fix_from_file() -> Result<Self, Box<dyn Error>> {
        let conf = Config::parse();

        let content = state::get_content(&conf)?;

        let delim = if conf.delim == "None" {
            get_delim(&content)?
        } else {
            eprintln!("got delimiter as arg");
            conf.delim.chars().next().unwrap()
        };

        eprintln!("Delimiter: \"{delim}\"");
        Ok(Config {
            delim: delim.to_string(),
            ..conf
        })
    }

    /// Path for statefile of filepath got, or if doesn't exist, self
    ///
    /// # Panics
    ///
    /// `get_prog_path()`
    pub fn file_path(&self) -> PathBuf {
        if state::prog_exists(&self.file_path_orig()) && !self.no_state {
            state::get_prog_path(&self.file_path_orig()).expect("Couldn't get progress path")
        } else {
            self.file_path.clone()
        }
    }

    /// Get original `file_path` as `PathBuf`
    pub fn file_path_orig(&self) -> PathBuf {
        self.file_path.clone()
    }

    /// Get delimiter as a character
    ///
    /// # Panics
    ///
    /// `delim` is empty
    pub fn delim(&self) -> char {
        self.delim.chars().next().unwrap()
    }
}

/// Get delimiter from content
fn get_delim(content: &str) -> Result<char, Box<dyn Error>> {
    const DELIMS: [char; 5] = [';', '|', '\t', '=', ':' /*',', '-'*/];

    if let Ok(delim) = get_prop(content, "delim") {
        return Ok(delim.chars().next().unwrap());
    }

    let mut delims_counts: HashMap<char, u32> = HashMap::new();

    for delim in &DELIMS {
        let mut delim_count = 0;
        content
            .lines()
            .filter(|line| !line.trim().starts_with('#') && !line.is_empty())
            .for_each(|line| delim_count += line.trim().chars().filter(|c| c == delim).count());
        if delim_count > 0 {
            delims_counts.insert(*delim, delim_count.try_into()?);
        }
    }
    for delim in &delims_counts {
        println!("'{}': {}", delim.0, delim.1);
    }
    if delims_counts.is_empty() {
        Err(format!("Couldn't determine delimiter, should be one of: {DELIMS:?}").into())
    } else {
        Ok(*delims_counts.iter().max_by_key(|x| x.1).unwrap().0)
    }
}

/// Get property from content
fn get_prop(content: &str, prop: &str) -> Result<String, Box<dyn Error>> {
    if content.contains("[crablit]") {
        eprintln!("text contains [crablit]!");
        let prop = &format!("{prop} = ");
        if !content.contains(prop) {
            eprintln!("Coudln't find \"{prop}\"");
            return Err(format!("Coudln't find \"{prop}\"").into());
        }
        Ok(content
            .lines()
            .find(|line| line.contains(prop))
            .unwrap()
            .split('=')
            .last()
            .unwrap()
            .trim()
            .trim_matches(|c| c == '"' || c == '\'')
            .into())
    } else {
        Err(format!("Coudln't find {prop}").into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_delim_config() {
        let content = "\
# [crablit]
# mode = \"cards\"
# delim = ':'

or : ||
and : &&
no command : ;;;;;; 
";
        assert_eq!(':', get_delim(content).unwrap());
    }
    #[test]
    fn get_delim_correct() {
        let content = "rot ; narancssárga";
        assert_eq!(';', get_delim(content).unwrap());
    }
    #[test]
    fn get_delim_hard() {
        let content = "barn\ta ; braun\nfluxus ; bohókás ármány";
        assert_eq!(';', get_delim(content).unwrap());
    }
    #[test]
    #[should_panic]
    fn get_delim_incorrect() {
        let content = "# barna , braun";
        assert_eq!(';', get_delim(content).unwrap());
    }

    // #[test]
    // fn configg() {
    //     let mut command = Command::cargo_bin("crablit").unwrap();
    //     command.arg("test.txt");
    // }

    //     #[test]
    //     fn basic_correct_cards() {
    //         let orig_conf = Config {
    //             file_path: "test.txt".to_owned(),
    //             card_swap: false,
    //             ask_both: false,
    //             mode: "".to_string(),
    //             delim: "".to_string(),
    //             no_shuffle: true,
    //         };
    //         let content = "\
    // # test deck, cards
    // term1 ; def1
    // term2 ; def2
    // ";
    //         assert_eq!(
    //             Config {
    //                 file_path: "test".to_string(),
    //                 card_swap: false,
    //                 ask_both: false,
    //                 mode: "cards".to_string(),
    //                 delim: ";".to_string(),
    //                 no_shuffle: true
    //             },
    //             Config::fix_from_file().unwrap()
    //         );
    //     }
}
