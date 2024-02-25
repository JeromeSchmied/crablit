//! # Library for vocabulary learning, used in `crablit`.
use crate::enums::{Msg, SPACER};
use colored::Colorize;
use rustyline::DefaultEditor;
use std::{
    error::Error,
    fmt::Debug,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::exit,
};

/// Module for learning Deck of Cards
pub mod cards;
/// Module for parsing cli arguments
pub mod config;
/// enums
pub mod enums;
/// Module for saving state: progress
pub mod state;
/// Module for learning Deck of Verbs
pub mod verbs;

// re-exports
pub use cards::Card;
pub use enums::Lok;
pub use enums::Mode;
pub use verbs::Verb;

/// The trait for learning either `Cards` of `Verbs`
pub trait Learn {
    /// Show as a term, waiting to be answered.
    fn question(&self) -> Msg;
    /// Show solution of term.
    fn correct(&self) -> String;
    /// Display message when skipping item.
    fn skip(&self) -> Msg;
    /// Display message when input of term is not correct.
    fn wrong(&self) -> Msg;
    /// Display flashcard.
    fn flashcard(&self) -> Msg;
    /// Show hint of term.
    fn hint(&self) -> Msg;
    /// Serialize: create instance of `Self` from a line from file containing vocab data.
    ///
    /// # Errors
    ///
    /// if couldn't find properly formatted line
    fn deser(line: &str, delim: char) -> Result<Box<Self>, Box<dyn Error>>;
    /// Deserialize: create a line of vocab data to be written to file from `self`
    fn ser(&self, delim: &str) -> String;
    /// increment knowledge level
    fn incr(&mut self);
    /// decrement knowledge level
    fn decr(&mut self);
    /// Lok
    fn lok(&self) -> Lok;
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
///
/// # Errors
///
/// - can't read `path`
/// - can't deserialize properly
pub fn init<T: Learn>(path: &PathBuf, delim: char) -> Result<Vec<T>, Box<dyn Error>> {
    // contents of file with vocab data
    let contents = fs::read_to_string(path)?;
    // results vector
    let mut r: Vec<T> = Vec::new();
    // iterating over the lines of file to store them in a vector
    for line in contents.lines() {
        // if is comment or empty
        if line.trim().starts_with('#') || line.is_empty() {
            continue;
        }
        r.push(*Learn::deser(line, delim)?);
    }
    eprintln!("File succesfully read.");
    // println!("content: {:?}", r);
    Ok(r)
}

/// Start learning the vector, return the remainders: ones not guessed correctly
///
/// # Errors
///
/// - `rustyline` can't create instance
pub fn question<T>(v: &mut [T], conf: &config::Config) -> Result<(), Box<dyn Error>>
where
    T: Learn + Debug + Clone,
{
    // let mut printer = String::new();
    println!(
        "\n\nYou have {} words to learn, let's start!\n\n",
        v.iter().filter(|item| item.lok() != Lok::Done).count()
    );
    let mut rl = DefaultEditor::new()?;

    let mut i = 0;
    while i < v.len() {
        let item = &mut v[i];

        if item.lok() == Lok::Done {
            i += 1;
            continue;
        }
        // display prompt
        let last_hr = rl.history().iter().last();
        // eprintln!("last history element: {:?}", last_hr);
        let msg = if last_hr.is_some_and(|he| {
            he.starts_with(":h") || he == ":typo" || he == ":n" || he == ":num" || he == ":togo"
        }) {
            format!("{}> ", enums::SPACER)
        } else {
            format!("{}\n{}> ", item.question().val(), enums::SPACER)
        };

        let guess = rl.readline(&msg)?;
        rl.add_history_entry(&guess)?;
        let guess = guess.trim();

        // is command
        if guess.starts_with(':') {
            match guess {
                ":q" | ":quit" | ":exit" => {
                    println!("{}", Msg::Exit.val());
                    exit(0);
                }

                ":h" | ":help" | ":hint" => {
                    println!("{}", item.hint().val());
                }

                ":w" | ":write" | ":save" => {
                    state::save_prog(v, conf)?;
                }

                ":wq" => {
                    state::save_prog(v, conf)?;
                    println!("{}", Msg::Exit.val());
                    exit(0);
                }

                ":typo" => {
                    // ask to type again before correcting?
                    if i > 0 {
                        if let Some(skipping) = v.get(i - 1) {
                            println!("{}", Msg::Typo(skipping.ser(" = ")).val());
                            v[i - 1].incr();
                        } else {
                            println!("{}", Msg::Typo("None".to_string()).val());
                        }
                    } else {
                        println!("{}", Msg::Typo("None".to_string()).val());
                    }
                    // rl.readline(&msg)?;
                }

                ":skip" => {
                    println!("{}\n\n", item.skip().val());
                    i += 1;
                    continue;
                }

                ":revise" => {
                    println!("{}", Msg::Revise.val());
                    break;
                }

                ":f" | ":flash" => {
                    println!("{}\n\n\n", item.flashcard().val(),);
                    item.incr();
                    i += 1;
                }

                ":n" | ":num" | ":togo" => {
                    println!("{}", &Msg::Togo(v.len(), i).val());
                }

                uc => {
                    println!("{} {}\n", "Unknown command:".red(), uc);
                }
            }
        } else if guess == item.correct() {
            println!("{}\n", Msg::Knew.val());
            item.incr();
            i += 1;
        } else {
            println!("{}", item.wrong().val());
            item.decr();
            i += 1;
        }
    }
    Ok(())
}

/// Starting program execution according to mode
///
/// # Errors
///
/// - `init()` returns an error
/// - `question()` returns an error
/// - `state::rm()` returns an error
/// - `verbs::deser_to_card()` returns an error
pub fn run(conf: &config::Config) -> Result<(), Box<dyn Error>> {
    match conf.mode() {
        Mode::Cards => {
            let mut v = init(&conf.file_path(), conf.delim())?;
            if conf.card_swap() {
                println!("swapping terms and definitions of each card");
                swap_cards(&mut v);
            }
            if conf.ask_both() {
                println!("swapping terms and definitions of some cards");
                randomly_swap_cards(&mut v);
            }

            while v.iter().filter(|item| item.lok() == Lok::Done).count() < v.len() {
                if !conf.no_shuffle() {
                    eprintln!("shuffling");
                    fastrand::shuffle(&mut v);
                }
                question(&mut v, conf)?;
            }
            println!("Gone through everything you wanted, great job!");
            state::rm_prog(&conf.file_path_orig())?;

            Ok(())
        }
        Mode::Verbs => {
            let mut v: Vec<Verb> = init(&conf.file_path(), conf.delim())?;
            println!(
                "\n\n\nStarting to learn verbs, input should be as following: <inf>, <dri>, <prä>, <per>"
            );

            // while !v.is_empty() {
            while v.iter().filter(|item| item.lok() == Lok::Done).count() < v.len() {
                eprintln!("shuffling");
                if !conf.no_shuffle() {
                    fastrand::shuffle(&mut v);
                }
                question(&mut v, conf)?;
            }
            println!("Gone through everything you wanted, great job!");
            state::rm_prog(&conf.file_path_orig())?;

            Ok(())
        }
        Mode::VerbsToCards => {
            let v: Vec<Verb> = init(&conf.file_path(), conf.delim())?;
            verbs::deser_to_card(&v, conf)?;

            Ok(())
        }
    }
}

/// Show hint from the string got
/// # usage
/// ```
/// use crablit::hint;
///
/// let dunno_want_hint = "This is a very-very hard-to-guess sentence.";
///
/// assert_eq!("This is a very-very h______________________", hint(dunno_want_hint));
///
/// let easy = "012345";
///
/// assert_eq!("012___", hint(easy));
/// ```
pub fn hint(s: &str) -> String {
    let n = s.chars().count() / 2;
    [
        s.chars().take(n).collect::<String>(),
        s.chars().skip(n).map(|_| '_').collect(),
    ]
    .concat()
}

/// Swap definition and term of deck(vector) of cards
///
/// # usage
/// ```
/// use crablit::Card;
///
/// let mut deck = vec![Card::new("term1", "def1", None), Card::new("term2", "def2", None), Card::new("term3", "def3", None)];
///
/// crablit::swap_cards(&mut deck);
/// ```
pub fn swap_cards(cards: &mut [cards::Card]) {
    cards.iter_mut().for_each(cards::Card::swap);
}

/// Randomly swap definition and term of deck(vector) of cards
///
/// # usage
/// ```
/// use crablit::Card;
///
/// let mut deck = vec![Card::new("term1", "def1", None), Card::new("term2", "def2", None), Card::new("term3", "def3", None)];
///
/// crablit::randomly_swap_cards(&mut deck);
/// ```
pub fn randomly_swap_cards(cards: &mut [cards::Card]) {
    for card in cards.iter_mut() {
        if fastrand::bool() {
            card.swap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn incorrect_mode() {
        let _ = Mode::from("mode");
    }
    #[test]
    fn correct_mode_cards() {
        assert_eq!(Mode::Cards, Mode::from("cards"));
    }
    #[test]
    fn mode_new_simple() {
        let mode = "verbs";
        assert_eq!(Mode::Verbs, Mode::from(mode));
    }
    #[test]
    fn mode_new_conv() {
        let mode = "verbs2cards";
        assert_eq!(Mode::VerbsToCards, Mode::from(mode));
        let mode = "convert";
        assert_eq!(Mode::VerbsToCards, Mode::from(mode));
    }

    #[test]
    fn hint_not_odd() {
        let get_hint = String::from("1234");
        assert_eq!("12__", hint(&get_hint));
    }
    #[test]
    fn hint_odd() {
        let get_hint = String::from("12345");
        assert_eq!("12___", hint(&get_hint));
    }
    #[test]
    fn hint_non_ascii() {
        let get_hint = String::from("aáéűúőóüöíä|Ä");
        assert_eq!("aáéűúő_______", hint(&get_hint));
    }

    #[test]
    fn swap_cards_works() {
        let mut cards = vec![Card::new("term", "definition", None)];

        swap_cards(&mut cards);
        assert_eq!(cards, vec![Card::new("definition", "term", None)]);
    }

    // init()
    // verbs::conv()
}
