//! # Library for vocabulary learning, used in `crablit`.
// dirs::data_dir()
use crate::{consts::*, verbs::Verbs};
use colored::{ColoredString, Colorize};
use nanorand::{Rng, WyRand};
use rustyline::DefaultEditor;
use std::{
    error::Error,
    fmt::Debug,
    fs::{self, File},
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
    fn new_from_line(line: &str, delim: char) -> Self;
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
        if s == "mode = verbs" || s == "verbs" || s == "mode = verb" || s == "verb" {
            Self::Verb
        } else if s == "mode = cards" || s == "cards" || s == "mode = card" || s == "card" {
            Self::Card
        } else if s == "mode = conv"
            || s == "conv"
            || s == "verb_conv"
            || s == "mode = convert"
            || s == "convert"
            || s == "cards2verbs"
            || s == "cardstoverbs"
            || s == "card2verb"
        {
            Self::VerbConv
        } else {
            panic!("Couldn't determine type of deck: it wasn't 'cards', 'verbs' or 'cards2verbs'!");
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
    let mut r: Vec<T> = Vec::new();
    let contents = fs::read_to_string(path)?;
    // iterating over the lines of file to store them in a vector
    for line in contents.lines() {
        let mut words = line.split(delim);
        let s = words.next().unwrap_or("").trim();
        // ignoring newlines, lines starting with #
        if s.is_empty() || s.starts_with('#') {
            continue;
        };
        r.push(Learn::new_from_line(line, delim));
    }
    eprintln!("File succesfully read.");
    // println!("content: {:?}", r);
    Ok(r)
}

/// Start learning the vector, return the remainders
pub fn question<T: Learn + Debug + Clone>(v: &[T]) -> Result<Vec<T>, Box<dyn Error>> {
    // let mut printer = String::new();
    if v.len() != 1 {
        println!("\n\nYou have {} words to learn, let's start!", v.len());
    }
    let mut r: Vec<T> = Vec::new();
    let mut rl = DefaultEditor::new()?;

    for elem in v {
        println!("{}", elem.show());
        // print!("{}> ", consts::SPACER);
        // io::stdout().flush().unwrap();

        let msg: &str = &format!("{}> ", consts::SPACER);
        let guess = rl.readline(msg).expect("Couldn't read rustyline");
        rl.add_history_entry(&guess)
            .expect("couldn't add to history");
        let guess = guess.trim();

        if guess == elem.correct() {
            println!("{} {}\n", Exp::Knew.val(), &Exp::KnewIt.val());
        } else if guess == ":skip" {
            println!("{}", elem.skip());
            continue;
        } else if guess == ":revise" {
            if r.len() == 1 {
                println!("Type revise again!");
            } else if r.is_empty() {
                println!("Nothing to revise, you might to type it again to make it work...");
            } else {
                println!("{}", Exp::Revise.val());
            }
            break;
        } else if guess == ":typo" {
            println!("{}{:?}", Exp::Typo.val(), r.pop());
            if !question(&[elem.clone()])?.is_empty() {
                r.push(elem.clone());
            }
        } else if guess == ":q" || guess == "quit" || guess == "exit" {
            println!("{}", Exp::Exit.val());
            exit(0);
        } else if guess == ":hint" || guess == ":h" {
            elem.hint();

            if !question(&[elem.clone()])?.is_empty() {
                r.push(elem.clone());
            }
        //treat them as flashcarding
        // } else if guess.is_empty() {
        //     println!("{} {}\n\n\n", Exp::val(&Exp::Flash), elem.flashcard(),);
        } else {
            r.push(elem.clone());
            println!("{}", elem.wrong());
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
    result = format!("{}{} ", result, Exp::Hint.val());
    let n = s.chars().count() / 2;
    for _ in 0..n {
        result = format!("{}{}", result, prt.next().unwrap());
    }
    result = format!(
        "{}{ch:_>widht$}",
        result,
        ch = '_',
        widht = s.chars().count() - n
    );
    result
}

/// Swap definition and term of deck of cards
fn swap_cards(cards: &mut [cards::Cards]) {
    cards.iter_mut().for_each(|card| card.swap());
}

/// Randomly swap definition and term of deck of cards
fn random_swap_cards(cards: &mut [cards::Cards]) {
    let mut rng = WyRand::new();
    cards.iter_mut().for_each(|card| {
        let swap: bool = rng.generate();
        if swap {
            card.swap()
        }
    });
}

/// Executing program core
pub fn run(config: &config::Config) -> Result<(), Box<dyn Error>> {
    let delim = config.delim.chars().next().unwrap();
    match Mode::new(&config.mode) {
        Mode::Card => {
            let mut v = init(&config.file_path, delim)?;
            if config.card_swap {
                println!("swapping terms and definitions of each card");
                swap_cards(&mut v);
            }
            if config.ask_both {
                println!("swapping terms and definitions of some cards");
                random_swap_cards(&mut v);
            }
            while !v.is_empty() {
                let mut rng = WyRand::new();
                if !config.no_shuffle {
                    eprintln!("shuffling");
                    rng.shuffle(&mut v);
                }
                v = question(&v)?;
            }

            println!("Gone through everything you wanted, great job!");
            Ok(())
        }
        Mode::Verb => {
            let mut v: Vec<Verbs> = init(&config.file_path, delim)?;
            println!(
                "\n\n\nStarting to learn verbs, input should be as following: <inf>, <dri>, <prä>, <per>"
            );
            while !v.is_empty() {
                let mut rng = WyRand::new();
                eprintln!("shuffling");
                if !config.no_shuffle {
                    rng.shuffle(&mut v);
                }
                v = question(&v)?;
            }
            println!("Gone through everything you wanted, great job!");
            Ok(())
        }
        Mode::VerbConv => {
            let v: Vec<Verbs> = init(&config.file_path, delim)?;
            println!(
                "\n\n\nConverting verbs to cards, from file: {:?} to file: {}",
                config.file_path,
                "verbs_as_cards.tsv".bright_blue()
            );
            verbs::conv(&v, "verbs_as_cards.tsv", '\t');
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::Cards;

    use super::*;

    #[test]
    fn swap_works() {
        let mut cards = vec![Cards::new("term", "definition")];

        swap_cards(&mut cards);
        assert_eq!(cards, vec![Cards::new("definition", "term")]);
    }

    #[test]
    fn hint_not_odd() {
        let get_hint = String::from("1234");
        assert_eq!(format!("{} 12__", Exp::Hint.val()), hint(&get_hint));
    }
    #[test]
    fn hint_odd() {
        let get_hint = String::from("12345");
        assert_eq!(format!("{} 12___", Exp::Hint.val()), hint(&get_hint));
    }
    #[test]
    fn hint_non_ascii() {
        let get_hint = String::from("aáéűúőóüöíä|Ä");
        assert_eq!(
            format!("{} aáéűúő_______", Exp::Hint.val()),
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
        let mode = "mode = verbs";
        assert_eq!(Mode::Verb, Mode::new(mode));
    }

    // init()
    // get_delim()
    // det_props()
    // verbs::conv()
}
