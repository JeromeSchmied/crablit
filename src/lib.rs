// dirs::data_dir()
use crate::consts::*;
use crate::verbs::Verbs;
use colored::{ColoredString, Colorize};
use nanorand::{Rng, WyRand};
use rustyline::DefaultEditor;
use std::{collections::HashMap, error::Error, fmt::Debug, fs, fs::File, process::exit};

/// Module for parsing cli arguments
pub mod args;
/// Module for learning Deck of Cards
pub mod cards;
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
    // fn copy(&self) -> Self;
}

#[derive(Debug, PartialEq)]
/// Type of Deck
pub enum Mode {
    Card,
    Verb,
    VerbConv,
}
impl Mode {
    pub fn new(s: &str) -> Self {
        let s = &s.to_lowercase();
        if s == "[mode: verbs]"
            || s == "verbs"
            || s == "[verbs]"
            || s == "[mode: verb]"
            || s == "verb"
            || s == "[verb]"
        {
            Mode::Verb
        } else if s == "[mode: cards]"
            || s == "cards"
            || s == "[cards]"
            || s == "[mode: card]"
            || s == "card"
            || s == "[card]"
        {
            Mode::Card
        } else if s == "[mode: conv]"
            || s == "conv"
            || s == "verb_conv"
            || s == "[mode: convert]"
            || s == "convert"
            || s == "cards2verbs"
            || s == "cardstoverbs"
            || s == "card2verb"
        {
            Mode::VerbConv
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

/// Get delimiter from a line
fn get_delim(content: &str) -> Result<char, String> {
    const DELIMS: [char; 5] = [';', '|', '\t', '=', ':' /*',', '-'*/];

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
    println!("{:?} file succesfully read.", path);
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
pub fn swap_cards(cards: &mut [cards::Cards]) {
    for card in cards {
        card.swap();
    }
}

/// Randomly swap definition and term of deck of cards
pub fn random_swap_cards(v: &mut [cards::Cards]) {
    for card in v {
        let mut rng = WyRand::new();
        let swap_terms: bool = rng.generate();
        if swap_terms {
            card.swap();
        }
    }
}

/// executing program core
pub fn run(args: &args::Config) -> Result<(), Box<dyn Error>> {
    let delim = args.delim.chars().next().unwrap();
    match Mode::new(&args.mode) {
        Mode::Card => {
            let mut v = init(&args.file_path, delim)?;
            if args.card_swap {
                println!("swapping terms and definitions of each card");
                swap_cards(&mut v);
            }
            if args.ask_both {
                println!("swapping terms and definitions of some cards");
                random_swap_cards(&mut v);
            }
            while !v.is_empty() {
                let mut rng = WyRand::new();
                if !args.no_shuffle {
                    eprintln!("shuffling");
                    rng.shuffle(&mut v);
                }
                v = question(&v)?;
            }

            println!("Gone through everything you wanted, great job!");
            Ok(())
        }
        Mode::Verb => {
            let mut v: Vec<Verbs> = init(&args.file_path, delim)?;
            println!(
                "\n\n\nStarting to learn verbs, input should be as following: <inf>, <dri>, <prä>, <per>"
            );
            while !v.is_empty() {
                let mut rng = WyRand::new();
                eprintln!("shuffling");
                if !args.no_shuffle {
                    rng.shuffle(&mut v);
                }
                v = question(&v)?;
            }
            println!("Gone through everything you wanted, great job!");
            Ok(())
        }
        Mode::VerbConv => {
            let v: Vec<Verbs> = init(&args.file_path, delim)?;
            println!(
                "\n\n\nConverting verbs to cards, from file: {:?} to file: {}",
                args.file_path,
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
    fn get_delim_correct() {
        let line = "rot ; narancssárga";
        assert_eq!(';', get_delim(line).unwrap());
    }
    #[test]
    fn get_delim_hard() {
        let line = "barn\ta ; braun\nfluxus ; bohókás ármány";
        assert_eq!(';', get_delim(line).unwrap());
    }
    #[test]
    #[should_panic]
    fn get_delim_incorrect() {
        let line = "# barna , braun";
        assert_eq!(';', get_delim(line).unwrap());
    }
    // init()
    // get_delim()
    // det_props()
    // verbs::conv()
}
