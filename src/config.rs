//! # In this module, you can find code that helps in collecting cli
//! arguments and determining properties of a file containing vocab data.
use clap::Parser;
use std::{collections::HashMap, error::Error, fs};

#[derive(Parser, Debug, PartialEq)]
#[command(version, about, author, long_about = None)]
pub struct Config {
    /// Path of the file to learn
    #[arg(required = true)]
    pub file_path: String,

    /// Swap terms and definitions of cards
    #[arg(short, long, default_value_t = false)]
    pub card_swap: bool,

    /// Sometimes ask the term, sometimes definition of cards
    #[arg(short, long, default_value_t = false)]
    pub ask_both: bool,

    /// Mode: either cards, verbs or verbs2cards
    #[arg(short, long, default_value = "cards")]
    pub mode: String,

    /// Delimiter used in file to seperate terms and definitions
    #[arg(short, long, default_value = ";")]
    pub delim: String,

    /// Don't shuffle card order
    #[arg(short, long, default_value_t = false)]
    pub no_shuffle: bool,
}

impl Config {
    /// Fixing properties by opening file that contains vocab data.
    pub fn fix_from_file() -> Result<Self, Box<dyn Error>> {
        let config = Config::parse();

        eprintln!("Trying to open {}", &config.file_path);
        let content = fs::read_to_string(&config.file_path)?;

        let delim = if config.delim != ";" {
            config.delim.chars().next().unwrap()
        } else {
            get_delim(&content)?
        };

        let mode = if config.mode != "cards" {
            config.mode
        } else {
            get_mode(&content, &delim)?
        };

        eprintln!("Mode: \"{}\", delimiter: \"{}\"", mode, delim);
        Ok(Config {
            file_path: config.file_path.clone(),
            card_swap: config.card_swap,
            ask_both: config.ask_both,
            no_shuffle: config.no_shuffle,
            mode,
            delim: delim.to_string(),
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
        .filter(|line| !line.starts_with('#') && !line.starts_with('\n') && !line.is_empty())
        .map(|line| sum += line.split(*delim).count())
        .count();

    let avg = (sum as f32 / n as f32).ceil();
    println!("sum: {sum}, n: {n}, avg: {avg}");
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
    for delim in DELIMS {
        let delim_count = content.chars().filter(|ch| ch == &delim).count();
        if delim_count > 0 {
            delims_counts.insert(delim, delim_count as u32);
        }
    }
    if delims_counts.is_empty() {
        Err(format!(
            "Couldn't determine delimiter type, should be one of: {:?}",
            DELIMS
        ))
    } else {
        let mut max: (char, u32) = ('\0', 0);
        for (k, v) in delims_counts {
            if v > max.1 {
                max = (k, v);
            }
        }
        Ok(max.0)
    }
}

/// Get property from content
fn get_prop(content: &str, prop: &str) -> Result<String, String> {
    if content.contains("[crablit]") {
        eprintln!("text contains [crablit]!");
        Ok(content
            .lines()
            .find(|line| line.contains(&format!("{} = ", prop)))
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
