//! # Library for vocabulary learning, used in `crablit`.
// dirs::data_dir()
use crate::{consts::*, verbs::Verb};
use colored::{ColoredString, Colorize};
use nanorand::{Rng, WyRand};
use rustyline::DefaultEditor;
use std::{
    error::Error,
    fmt::Debug,
    fs::{self, File},
    io::Write,
    process::exit,
};

/// Module for learning Deck of Cards
pub mod cards;
/// Module for parsing cli arguments
pub mod config;
/// commonly used expressions(text), colored strings
pub mod consts;
/// Module for learning Deck of Verbs
pub mod verbs;

/// The trait for learning either `Cards` of `Verbs`
pub trait Learn {
    fn show(&self) -> String;
    fn correct(&self) -> String;
    fn skip(&self) -> String;
    fn wrong(&self) -> String;
    fn flashcard(&self) -> String;
    fn hint(&self);
    fn serialize(line: &str, delim: char) -> Result<Box<Self>, String>;
    fn deserialize<T: Learn>(&self, v: &[T]) -> Result<String, Box<dyn Error>>;
}

#[derive(Debug, PartialEq)]
/// Type of Deck
pub enum Mode {
    Card,
    Verb,
    VerbConv,
}
impl Mode {
    /// Creates new instance of `Self`
    /// # usage
    /// ```
    /// use crablit::Mode;
    ///
    /// let mode = Mode::new("verbs");
    ///
    /// assert_eq!(mode, Mode::Verb);
    /// ```
    /// # panics
    /// if mode is neither verbs, cards, or verbs2cards
    pub fn new(mode: &str) -> Self {
        let s = &mode.to_lowercase();
        if s == "verbs" || s == "verb" {
            Self::Verb
        } else if s == "cards" || s == "card" {
            Self::Card
        } else if s == "conv" || s == "convert" || s == "verb_conv" || s == "verbs2cards" {
            Self::VerbConv
        } else {
            panic!("Couldn't determine type of deck: it wasn't 'cards', 'verbs' or 'verbs2cards'!");
        }
    }
}

// enum Kards {
//     Adjektiv(String),
//     Nomen(String),
//     Verb {
//         inf: String,
//         dri: String,
//         pra: String,
//         per: String,
//     },
//     Wendungen(String),
// }

/// Initializing deck of either `cards`, or `verbs`
pub fn init<T: Learn + Debug + Clone>(path: &str, delim: char) -> Result<Vec<T>, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let mut r: Vec<T> = Vec::new();
    // iterating over the lines of file to store them in a vector
    for line in contents.lines() {
        if line.trim().starts_with('#') || line.is_empty() {
            continue;
        }
        r.push(*Learn::serialize(line, delim)?);
    }
    eprintln!("File succesfully read.");
    // println!("content: {:?}", r);
    Ok(r)
}

/// Start learning the vector, return the remainders
pub fn question<T: Learn + Debug + Clone>(
    v: &[T],
    conf: &config::Config,
) -> Result<Vec<T>, Box<dyn Error>> {
    // let mut printer = String::new();
    if v.len() != 1 {
        println!("\n\nYou have {} words to learn, let's start!", v.len());
    }
    let mut r: Vec<T> = Vec::new();
    let mut rl = DefaultEditor::new()?;

    for elem in v {
        println!("{}", elem.show());
        // print!("{}> ", consts::SPACER);
        // io::stdout().flush()?;

        let msg = &format!("{}> ", consts::SPACER);
        let guess = rl.readline(msg)?;
        rl.add_history_entry(&guess)
            .expect("couldn't add to history");
        let guess = guess.trim();

        match guess {
            s if s == elem.correct() => println!("{} {}\n", Msg::Knew.val(), &Msg::KnewIt.val()),

            ":q" | "quit" | "exit" => {
                println!("{}", Msg::Exit.val());
                exit(0);
            }

            ":h" | ":hint" => {
                elem.hint();
                if !question(&[elem.clone()], conf)?.is_empty() {
                    r.push(elem.clone());
                }
            }

            ":w" | ":write" | ":save" => {
                let state_file_path = &format!("{}{}", STATE_HOME, &conf.file_path);

                let mut ofile = File::create(state_file_path)?;

                writeln!(ofile, "# [crablit]")?;
                writeln!(ofile, "# mode = \"cards\"")?;
                writeln!(ofile, "# delim = \'{}\'\n\n", conf.delim)?;

                let content = elem.deserialize(&r)?;
                writeln!(ofile, "{}", content)?;

                eprintln!("Saved file to {}{}.", SPACER, state_file_path);
            }

            ":typo" => {
                // ask to type before correcting
                println!("{}{:?}", Msg::Typo.val(), r.pop());
                if !question(&[elem.clone()], conf)?.is_empty() {
                    r.push(elem.clone());
                }
            }

            ":skip" => {
                println!("{}", elem.skip());
                continue;
            }

            ":revise" => {
                if r.len() == 1 {
                    println!("Type revise again!");
                } else if r.is_empty() {
                    println!("Nothing to revise, you might to type it again to make it work...");
                } else {
                    println!("{}", Msg::Revise.val());
                }
                break;
            }

            ":flash" => {
                //     println!("{} {}\n\n\n", &Msg::Flash.val(), elem.flashcard(),);
                todo!();
            }

            _ => {
                r.push(elem.clone());
                println!("{}", elem.wrong());
            }
        }
    }
    if r.len() > 1 {
        println!("\n\n{} remaining cards are {:#?}", r.len(), r);
    }
    Ok(r)
}

/// Show hint from the string got
fn hint(s: &str) -> String {
    let mut result = String::new();
    let mut prt = s.chars();
    result = format!("{}{} ", result, Msg::Hint.val());
    let n = s.chars().count() / 2;
    (0..n).for_each(|_| result = format!("{}{}", result, prt.next().unwrap()));
    result = format!(
        "{}{ch:_>widht$}",
        result,
        ch = '_',
        widht = s.chars().count() - n
    );
    result
}

/// Swap definition and term of deck of cards
fn swap_cards(cards: &mut [cards::Card]) {
    cards.iter_mut().for_each(|card| card.swap());
}

/// Randomly swap definition and term of deck of cards
fn randomly_swap_cards(cards: &mut [cards::Card]) {
    let mut rng = WyRand::new();
    cards.iter_mut().for_each(|card| {
        let swap: bool = rng.generate();
        if swap {
            card.swap()
        }
    });
}

/// Executing program core
pub fn run(conf: &config::Config) -> Result<(), Box<dyn Error>> {
    let delim = conf.delim.chars().next().unwrap();
    match Mode::new(&conf.mode) {
        Mode::Card => {
            let mut v = init(&conf.file_path, delim)?;
            if conf.card_swap {
                println!("swapping terms and definitions of each card");
                swap_cards(&mut v);
            }
            if conf.ask_both {
                println!("swapping terms and definitions of some cards");
                randomly_swap_cards(&mut v);
            }
            while !v.is_empty() {
                let mut rng = WyRand::new();
                if !conf.no_shuffle {
                    eprintln!("shuffling");
                    rng.shuffle(&mut v);
                }
                v = question(&v, conf)?;
            }

            println!("Gone through everything you wanted, great job!");

            Ok(())
        }
        Mode::Verb => {
            let mut v: Vec<Verb> = init(&conf.file_path, delim)?;
            println!(
                "\n\n\nStarting to learn verbs, input should be as following: <inf>, <dri>, <prä>, <per>"
            );
            while !v.is_empty() {
                let mut rng = WyRand::new();
                eprintln!("shuffling");
                if !conf.no_shuffle {
                    rng.shuffle(&mut v);
                }
                v = question(&v, conf)?;
            }
            println!("Gone through everything you wanted, great job!");

            Ok(())
        }
        Mode::VerbConv => {
            let v: Vec<Verb> = init(&conf.file_path, delim)?;
            verbs::deser_to_conv(&v, conf)?;

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::Card;

    use super::*;

    #[test]
    fn swap_works() {
        let mut cards = vec![Card::new("term", "definition")];

        swap_cards(&mut cards);
        assert_eq!(cards, vec![Card::new("definition", "term")]);
    }

    #[test]
    fn hint_not_odd() {
        let get_hint = String::from("1234");
        assert_eq!(format!("{} 12__", Msg::Hint.val()), hint(&get_hint));
    }
    #[test]
    fn hint_odd() {
        let get_hint = String::from("12345");
        assert_eq!(format!("{} 12___", Msg::Hint.val()), hint(&get_hint));
    }
    #[test]
    fn hint_non_ascii() {
        let get_hint = String::from("aáéűúőóüöíä|Ä");
        assert_eq!(
            format!("{} aáéűúő_______", Msg::Hint.val()),
            hint(&get_hint)
        );
    }

    #[test]
    #[should_panic]
    fn incorrect_mode() {
        Mode::new("mode");
    }
    #[test]
    fn correct_mode_cards() {
        assert_eq!(Mode::Card, Mode::new("cards"));
    }

    #[test]
    fn mode_new_simple() {
        let mode = "verbs";
        assert_eq!(Mode::Verb, Mode::new(mode));
    }
    #[test]
    fn mode_new_in_config() {
        let mode = "verbs2cards";
        assert_eq!(Mode::VerbConv, Mode::new(mode));
    }

    // init()
    // verbs::conv()
}
