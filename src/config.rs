//! # In this module, you can find code that helps in collecting cli
//! arguments and determining properties of a file containing vocab data.
use crate::*;
use clap::Parser;
use std::{collections::HashMap, error::Error, fs, path::PathBuf};

#[derive(Parser, Debug, PartialEq)]
#[command(version, about, author, long_about = None)]
pub struct Config {
    /// Path of the file to learn
    #[arg(required = true)]
    file_path: String,

    /// Swap terms and definitions of cards
    #[arg(short = 's', long, default_value_t = false)]
    card_swap: bool,

    /// Sometimes ask the term, sometimes definition of cards
    #[arg(short, long, default_value_t = false)]
    ask_both: bool,

    /// Mode: either cards, verbs or verbs2cards
    #[arg(short, long, default_value = "None")]
    mode: String,

    /// Delimiter used in file to seperate terms and definitions
    #[arg(short, long, default_value = "None")]
    delim: String,

    /// Don't shuffle card order
    #[arg(long, default_value_t = false)]
    no_shuffle: bool,

    /// Don't load previous state
    #[arg(long, default_value_t = false)]
    no_state: bool,

    /// Only check file syntax don't actually start learning deck
    #[arg(short = 'c', long = "check", default_value_t = false)]
    only_check: bool,
}

impl Config {
    /// Fixing properties by opening file that contains vocab data.
    pub fn fix_from_file() -> Result<Self, Box<dyn Error>> {
        let conf = Config::parse();

        let state_file_path = state::get_prog_path(&conf.file_path_orig())?;
        println!("searching for path at: {:?}", state_file_path);
        let content = if !conf.no_state && crate::state::prog_exists(&conf.file_path_orig()) {
            let state_file_path = state::get_prog_path(&conf.file_path_orig())?;

            eprintln!(
                "Opening file from previously saved state: \"{:?}\"",
                &state_file_path
            );

            let state_file = fs::read_to_string(&state_file_path)?;
            println!("state file content:\n{:?}\n", state_file);
            state_file
        } else {
            eprintln!("Trying to open {}", &conf.file_path);

            fs::read_to_string(&conf.file_path)?
        };

        let delim = if conf.delim != "None" {
            eprintln!("got delimiter as arg");
            conf.delim.chars().next().unwrap()
        } else {
            get_delim(&content)?
        };

        let mode = if conf.mode != "None" {
            eprintln!("got mode as arg");
            conf.mode
        } else {
            get_mode(&content, &delim)?.disp()
        };

        eprintln!("Mode: \"{}\", delimiter: \"{}\"", mode, delim);
        Ok(Config {
            file_path: conf.file_path,
            card_swap: conf.card_swap,
            ask_both: conf.ask_both,
            no_shuffle: conf.no_shuffle,
            mode,
            delim: delim.to_string(),
            only_check: conf.only_check,
            no_state: conf.no_state,
        })
    }

    /// Path for statefile of filepath got, or if doesn't exist, self
    pub fn file_path(&self) -> PathBuf {
        if state::prog_exists(&self.file_path_orig()) && !self.no_state {
            state::get_prog_path(&self.file_path_orig()).expect("Coudln't get progress path")
        } else {
            self.file_path.clone().into()
        }
    }

    /// Get original file_path as PathBuf
    pub fn file_path_orig(&self) -> PathBuf {
        self.file_path.clone().into()
    }

    /// Get no_state
    pub fn no_state(&self) -> bool {
        self.no_state
    }

    /// Get no_shuffle
    pub fn no_shuffle(&self) -> bool {
        self.no_shuffle
    }

    /// Get only_check
    pub fn only_check(&self) -> bool {
        self.only_check
    }

    /// Get ask_both
    pub fn ask_both(&self) -> bool {
        self.ask_both
    }

    /// Get card_swap
    pub fn card_swap(&self) -> bool {
        self.card_swap
    }

    /// Get mode as `Mode`
    pub fn mode(&self) -> Mode {
        Mode::from(&self.mode)
    }

    /// Get delimiter as a character
    pub fn delim(&self) -> char {
        self.delim.chars().next().unwrap()
    }
}

/// Get mode from content
fn get_mode(content: &str, delim: &char) -> Result<Mode, &'static str> {
    if let Ok(mode) = get_prop(content, "mode") {
        return Ok(Mode::from(&mode));
    }
    // get avg count of splits
    let mut sum = 0;
    let n = content
        .lines()
        .filter(|line| !line.trim().starts_with('#') && !line.is_empty())
        .map(|line| sum += line.split(*delim).count())
        .count();

    let avg = (sum as f32 / n as f32).ceil() as u8;
    eprintln!("sum: {sum}, n: {n}, avg: {avg}");
    if avg == 2 {
        Ok(Mode::Cards)
    } else if avg > 2 && avg < 7 {
        Ok(Mode::Verbs)
    } else {
        Err("couldn't determine mode of deck")
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
            delims_counts.insert(*delim, delim_count as u32);
        }
    }
    for delim in &delims_counts {
        println!("'{}': {}", delim.0, delim.1);
    }
    if delims_counts.is_empty() {
        Err(format!(
            "Couldn't determine delimiter, should be one of: {:?}",
            DELIMS
        )
        .into())
    } else {
        Ok(*delims_counts.iter().max_by_key(|x| x.1).unwrap().0)
    }
}

/// Get property from content
fn get_prop(content: &str, prop: &str) -> Result<String, Box<dyn Error>> {
    if content.contains("[crablit]") {
        eprintln!("text contains [crablit]!");
        let prop = &format!("{} = ", prop);
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
        Err(format!("Coudln't find {}", prop).into())
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

    #[test]
    fn get_mode_config() {
        let content = "\
# [crablit]
# mode = \"cards\"
# delim = ':'

or : ||
and : &&
no command : ;;;;;; 
";
        assert_eq!(get_mode(content, &':'), Ok(Mode::Cards));
    }

    #[test]
    fn get_mode_correct_simple() {
        let content = "term ; condition";
        assert_eq!(get_mode(content, &';'), Ok(Mode::Cards));
    }

    #[test]
    fn get_mode_correct_complex() {
        let content = "\
# mode = \"cards\"
# delim = ':'

or : ||
and : &&
no command : ;;;;;; 
";
        assert_eq!(get_mode(content, &':'), Ok(Mode::Cards));
    }

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
