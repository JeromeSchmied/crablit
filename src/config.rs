//! # In this module, you can find code that helps in collecting cli
//! arguments and determining properties of a file containing vocab data.
use clap::Parser;
use std::{collections::HashMap, error::Error, fs};

use crate::state::{get_state_path, progress_exists};

#[derive(Parser, Debug, PartialEq)]
#[command(version, about, author, long_about = None)]
pub struct Config {
    /// Path of the file to learn
    #[arg(required = true)]
    pub file_path: String,

    /// Swap terms and definitions of cards
    #[arg(short = 's', long, default_value_t = false)]
    pub card_swap: bool,

    /// Sometimes ask the term, sometimes definition of cards
    #[arg(short, long, default_value_t = false)]
    pub ask_both: bool,

    /// Mode: either cards, verbs or verbs2cards
    #[arg(short, long, default_value = "None")]
    pub mode: String,

    /// Delimiter used in file to seperate terms and definitions
    #[arg(short, long, default_value = "None")]
    pub delim: String,

    /// Don't shuffle card order
    #[arg(long, default_value_t = false)]
    pub no_shuffle: bool,

    /// Don't load previous state
    #[arg(long, default_value_t = false)]
    pub no_state: bool,

    /// Only check file syntax don't actually start learning deck
    #[arg(short = 'c', long = "check", default_value_t = false)]
    pub only_check: bool,
}

impl Config {
    /// Fixing properties by opening file that contains vocab data.
    pub fn fix_from_file() -> Result<Self, Box<dyn Error>> {
        let conf = Config::parse();

        let (content, fpath) = if !conf.no_state && progress_exists(&conf.file_path) {
            let state_file_path = get_state_path(&conf.file_path)?;

            eprintln!("Searching for state at: \"{}\"", &state_file_path);
            eprintln!(
                "Opening file from previously saved state: \"{}\"",
                &state_file_path
            );

            let state_file = fs::read_to_string(&state_file_path)?;
            println!("state file content:\n{:?}\n", state_file);
            (state_file, state_file_path.to_string())
        } else {
            eprintln!("Trying to open {}", &conf.file_path);
            let content = fs::read_to_string(&conf.file_path)?;
            (content, conf.file_path)
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
            get_mode(&content, &delim)?
        };

        eprintln!("Mode: \"{}\", delimiter: \"{}\"", mode, delim);
        Ok(Config {
            file_path: fpath.to_string(),
            card_swap: conf.card_swap,
            ask_both: conf.ask_both,
            no_shuffle: conf.no_shuffle,
            mode,
            delim: delim.to_string(),
            only_check: conf.only_check,
            no_state: conf.no_state,
        })
    }
}

/// Get mode from content
fn get_mode(content: &str, delim: &char) -> Result<String, &'static str> {
    if let Ok(mode) = get_prop(content, "mode") {
        return Ok(mode);
    }
    // get avg count of splits
    let mut sum = 0;
    let n = content
        .lines()
        .filter(|line| !line.trim().starts_with('#') && !line.is_empty())
        .map(|line| sum += line.split(*delim).count())
        .count();

    let avg = (sum as f32 / n as f32).ceil();
    eprintln!("sum: {sum}, n: {n}, avg: {avg}");
    if avg == 2. {
        Ok("cards".to_string())
    } else if avg > 2. && avg < 7. {
        Ok("verbs".to_string())
    } else {
        Err("couldn't determine mode of deck")
    }
}

/// Get delimiter from content
fn get_delim(content: &str) -> Result<char, String> {
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
        println!("{}: {}", delim.0, delim.1);
    }
    if delims_counts.is_empty() {
        Err(format!(
            "Couldn't determine delimiter, should be one of: {:?}",
            DELIMS
        ))
    } else {
        Ok(*delims_counts.iter().max_by_key(|x| x.1).unwrap().0)
    }
}

/// Get property from content
fn get_prop(content: &str, prop: &str) -> Result<String, String> {
    if content.contains("[crablit]") {
        eprintln!("text contains [crablit]!");
        let prop = &format!("{} = ", prop);
        if !content.contains(prop) {
            eprintln!("Coudln't find \"{prop}\"");
            return Err(format!("Coudln't find \"{prop}\""));
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
            .to_string())
    } else {
        Err(format!("Coudln't find {}", prop))
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
        assert_eq!(Ok(':'), get_delim(content));
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
        assert_eq!(get_mode(content, &':'), Ok("cards".to_string()));
    }

    #[test]
    fn get_mode_correct_simple() {
        let content = "term ; condition";
        assert_eq!(get_mode(content, &';'), Ok("cards".to_string()));
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
        assert_eq!(get_mode(content, &':'), Ok("cards".to_string()));
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
