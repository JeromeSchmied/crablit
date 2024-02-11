//! # Library for vocabulary learning, used in `crablit`.
use crate::expressions::*;
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
/// Commonly used expressions(text), colored strings
pub mod expressions;
/// Module for saving state: progress
pub mod state;
/// Module for learning Deck of Verbs
pub mod verbs;

// re-exports
pub use cards::Card;
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
    fn deser(line: &str, delim: char) -> Result<Box<Self>, Box<dyn Error>>;
    /// Deserialize: create a line of vocab data to be written to file from `self`
    fn ser(&self, delim: &str) -> String;
}

#[derive(Debug, PartialEq)]
/// Type of Deck
pub enum Mode {
    /// Basic term-definition
    Cards,
    /// More complex: for learning verbforms
    Verbs,
    /// Convert from `Verbs` to `Cards`. term as term, infinitive as definition.
    VerbsToCards,
}
impl Mode {
    /// Creates new instance of `Self`
    /// # usage
    /// ```
    /// use crablit::Mode;
    ///
    /// let mode = Mode::from("verbs");
    ///
    /// assert_eq!(mode, Mode::Verbs);
    /// ```
    /// # panics
    /// if mode is neither verbs, cards, or conv
    pub fn from(mode: &str) -> Self {
        let s = &mode.to_lowercase();
        if s == "verbs" || s == "verb" {
            Self::Verbs
        } else if s == "cards" || s == "card" {
            Self::Cards
        } else if s == "conv" || s == "convert" || s == "verb_conv" || s == "verbs2cards" {
            Self::VerbsToCards
        } else {
            panic!("Couldn't determine type of deck: it wasn't 'cards', 'verbs' or 'verbs2cards'!");
        }
    }

    /// Creates conviniently displayable String
    /// # usage
    /// ```
    /// use crablit::Mode;
    ///
    /// let mode = Mode::from("convert");
    ///
    /// assert_eq!("convert", mode.disp())
    /// ```
    pub fn disp(&self) -> String {
        match self {
            Mode::Cards => "cards".into(),
            Mode::Verbs => "verbs".into(),
            Mode::VerbsToCards => "convert".into(),
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
pub fn question<T>(v: &[T], conf: &config::Config) -> Result<Vec<T>, Box<dyn Error>>
where
    T: Learn + Debug + Clone,
{
    // let mut printer = String::new();
    println!("\n\nYou have {} words to learn, let's start!\n\n", v.len());
    // results vector
    let mut r: Vec<T> = Vec::new();

    let mut rl = DefaultEditor::new()?;

    let mut i = 0;
    while i < v.len() {
        let item = &v[i];
        // display prompt
        let last_hr = rl.history().iter().last();
        // eprintln!("last history element: {:?}", last_hr);
        let msg = if last_hr.is_some_and(|he| {
            he.starts_with(":h") || he == ":typo" || he == ":n" || he == ":num" || he == ":togo"
        }) {
            format!("{}> ", expressions::SPACER)
        } else {
            format!("{}\n{}> ", item.question().val(), expressions::SPACER)
        };

        // let msg = format!("\n{}\n{}> ", item.question(), expressions::SPACER);
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
                    state::save_prog(&r, conf)?;
                }

                ":wq" => {
                    state::save_prog(&r, conf)?;
                    println!("{}", Msg::Exit.val());
                    exit(0);
                }

                ":typo" => {
                    // ask to type again before correcting?
                    if let Some(skipping) = r.pop() {
                        println!("{}", Msg::Typo(skipping.ser(" = ")).val());
                    } else {
                        println!("{}", Msg::Typo("None".to_string()).val())
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
                    i += 1;
                    // todo!();
                }

                ":n" | ":num" | ":togo" => {
                    println!("{}", &Msg::Togo(v.len(), i).val());
                }

                uc => {
                    // return Err(["unknown command: ", uc].concat().into());
                    println!("{} {}\n", "Unknown command:".red(), uc);
                }
            }
        } else if guess == item.correct() {
            println!("{}\n", Msg::Knew.val());
            i += 1;
        } else {
            r.push(item.clone());
            println!("{}", item.wrong().val());
            i += 1;
        }
    }
    println!("\n\n{} remaining cards are {:#?}", r.len(), r);
    Ok(r)
}

/// Starting program execution according to mode
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

            while !v.is_empty() {
                if !conf.no_shuffle() {
                    eprintln!("shuffling");
                    fastrand::shuffle(&mut v);
                }
                v = question(&v, conf)?;
            }

            state::rm_prog(&conf.file_path_orig())?;

            println!("Gone through everything you wanted, great job!");

            Ok(())
        }
        Mode::Verbs => {
            let mut v: Vec<Verb> = init(&conf.file_path(), conf.delim())?;
            println!(
                "\n\n\nStarting to learn verbs, input should be as following: <inf>, <dri>, <prä>, <per>"
            );

            while !v.is_empty() {
                eprintln!("shuffling");
                if !conf.no_shuffle() {
                    fastrand::shuffle(&mut v);
                }
                v = question(&v, conf)?;
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
/// let mut deck = vec![Card::new("term1", "def1"), Card::new("term2", "def2"), Card::new("term3", "def3")];
///
/// crablit::swap_cards(&mut deck);
/// ```
pub fn swap_cards(cards: &mut [cards::Card]) {
    cards.iter_mut().for_each(|card| card.swap());
}

/// Randomly swap definition and term of deck(vector) of cards
///
/// # usage
/// ```
/// use crablit::Card;
///
/// let mut deck = vec![Card::new("term1", "def1"), Card::new("term2", "def2"), Card::new("term3", "def3")];
///
/// crablit::randomly_swap_cards(&mut deck);
/// ```
pub fn randomly_swap_cards(cards: &mut [cards::Card]) {
    cards.iter_mut().for_each(|card| {
        if fastrand::bool() {
            card.swap()
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn incorrect_mode() {
        Mode::from("mode");
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
        let mut cards = vec![Card::new("term", "definition")];

        swap_cards(&mut cards);
        assert_eq!(cards, vec![Card::new("definition", "term")]);
    }

    // init()
    // verbs::conv()
}
