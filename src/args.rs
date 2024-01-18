use crate::*;
use clap::Parser;
use std::{error::Error, fs};

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
    pub fn fix_from_file() -> Result<Self, Box<dyn Error>> {
        let config = args::Config::parse();
        eprintln!("Trying to open {}", &config.file_path);
        let content = fs::read_to_string(&config.file_path)?;

        let delim = if config.delim != ";" {
            config.delim.chars().next().unwrap()
        } else {
            get_delim(&content)?
        };
        // let mut limes = content.lines();
        // let mut prev = '\0';
        // let mut avg_delims = 0;

        // loop {
        //     let line = &limes.next().unwrap_or("");
        //     if !(line.is_empty() || line.starts_with('#')) {
        //         delim = get_delim(line);
        //         if delim == prev {
        //             break;
        //         }
        //         prev = delim;
        //     }
        // }

        let mode = if config.mode != "cards" {
            config.mode
        } else if content
            .lines()
            .nth(content.lines().count() / 2)
            .unwrap()
            .split(delim)
            .count()
            > 2
        {
            "verbs".to_string()
        } else {
            "cards".to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_correct_cards() {
        let orig_conf = Config {
            file_path: "test.txt".to_owned(),
            card_swap: false,
            ask_both: false,
            mode: "".to_string(),
            delim: "".to_string(),
            no_shuffle: true,
        };
        let content = "\
# test deck, cards
term1 ; def1
term2 ; def2
";
        assert_eq!(
            Config {
                file_path: "test".to_string(),
                card_swap: false,
                ask_both: false,
                mode: "cards".to_string(),
                delim: ";".to_string(),
                no_shuffle: true
            },
            Config::fix_from_file().unwrap()
        );
    }
}
