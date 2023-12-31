// dirs::data_dir()
use crate::consts::*;
use colored::{ColoredString, Colorize};
use nanorand::{Rng, WyRand};
use rustyline::DefaultEditor;
use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
    process::exit,
};

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

#[derive(Debug)]
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

/// Take input from console with `rustyline`
fn user_input(msg: &str) -> String {
    let mut rl = DefaultEditor::new().expect("Couldn't init rl");
    rl.readline(msg).expect("Couldn't read rustyline")
    // rl.add_history_entry(line)
}

/// Determine delimiter, type of Deck
pub fn determine_properties(path: &str) -> (Mode, char, u8) {
    println!("Trying to open {:?}", &path);
    let f = File::open(path).expect("couldn't open file");
    // getting contents of file
    let mut br = BufReader::new(f);
    let mut limes = String::new();
    // storing lines of contents
    br.read_to_string(&mut limes).expect("couldnt Read");

    // let mut contents = fs::read_to_string(&path).unwrap_or("redo".to_string());
    // let mut path_fixed = path.to_owned();
    // loop {
    //     if content == "redo" {
    //         println!("No luck opening the file...");
    //         path_fixed = user_input("Sorry, what is the correct path?");
    //         content = fs::read_to_string(&path_fixed).unwrap_or("redo".to_string());
    //     } else {
    //         break;
    //     }
    //     println!("Trying to open {:?}", path_fixed);
    // }
    let delim: char;
    let mode: String;
    // plus one for [crablit], one for an extra newline at the end
    let mut num = 2;

    // let mut limes = br.lines().flatten();
    let mut limes = limes.lines();
    // checking wether first line includes [crablit] to know if it is made for crablit
    if limes.next().unwrap() == "[crablit]" {
        mode = limes.next().unwrap_or("cards").to_string();
        num += 1;
        delim = limes
            .next()
            .unwrap_or(";")
            .chars()
            .nth_back(1)
            .unwrap_or(';');
        // mode = limes.next().unwrap_or_else(|| user_input("Mode?"));
        num += 1;
    } else {
        let last = limes.clone().last().unwrap_or("");
        loop {
            let line = &limes.next().unwrap_or("");
            if !(line.is_empty() || line.starts_with('#')) && get_delim(line) == get_delim(last) {
                delim = get_delim(line);
                break;
            }
        }
        // mode = user_input("Mode(cards/verbs)?").to_string();
        mode = "cards".to_owned();
        // resetting lines to start from beginning
        num = 0;
    }
    println!(
        "Mode: \"{}\", delimiter: \"{}\", number of lines skipping: \"{}\"",
        mode, delim, num
    );
    (Mode::new(&mode), delim, num)
}

/// Get delimiter from a line
fn get_delim(line: &str) -> char {
    const DELIMS: [char; 6] = [';', '|', '\t', ':', ',', '-'];
    // let line = line.unwrap_or("".to_owned());
    for delim in DELIMS {
        if !(line.is_empty() || line.starts_with('#')) && line.chars().any(|x| x == delim) {
            return delim;
        }
    }
    // asking for user input as delimiter is unknown
    let mut dlim = user_input("What character is the delimiter? ");
    if dlim.ends_with('\n') {
        dlim.pop();
    }
    dlim.chars().next().unwrap_or(';')
}

/// Initializing deck of either `cards`, or `verbs`
pub fn init<T: Learn + Debug + Clone>(path: &Path, delim: char, n: u8) -> Vec<T> {
    let mut r: Vec<T> = Vec::new();
    let br = BufReader::new(File::open(path).expect("Couldn't open file, quitting..."));
    // let contents = fs::read_to_string(path).expect("what is wronk?");
    // storing lines of contents
    // let mut limes = contents.lines();
    let mut lines = br.lines();
    for _ in 0..n {
        lines.next();
    }
    // iterating over the lines of file to store them in a vector
    for line in lines {
        let line = line.expect("Something wrong with bufread line");
        // dbg!(&line);
        let mut words = line.split(delim);
        let s = words.next().unwrap_or("").trim();
        // ignoring newlines, lines starting with #
        if s.is_empty() || s.starts_with('#') {
            continue;
        };
        r.push(Learn::new_from_line(&line, delim));
    }
    println!("{:?} file succesfully read.", path);
    // println!("content: {:?}", r);
    r
}

/// Start learning the vector, return the remainders
pub fn question<T: Learn + Debug + Clone>(v: Vec<T>) -> Vec<T> {
    // let mut printer = String::new();
    if v.len() != 1 {
        println!("\n\nYou have {} words to learn, let's start!", v.len());
    }
    let mut r: Vec<T> = Vec::new();

    for elem in &v {
        // if defi.is_empty() || defi == "NO_DEFINITION" || term.is_empty() || term == "NO_TERM" {
        //     println!("{}", "Oh, no! Missing word found!".bright_red());
        //     dbg!(&defi);
        //     dbg!(&term);
        //     continue;
        // }
        println!("{}", elem.show());

        let guess = user_input(&format!("{}> ", consts::SPACER));
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
            if !question(vec![elem.clone()]).is_empty() {
                r.push(elem.clone());
            }
        } else if guess == ":q" || guess == "quit" || guess == "exit" {
            println!("{}", Exp::Exit.val());
            exit(0);
        } else if guess == ":hint" || guess == ":h" {
            elem.hint();

            if !question(vec![elem.clone()]).is_empty() {
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
    r
}

/// Show hint from the string got
fn hint(s: &str) {
    let mut prt = s.chars();
    print!("{} ", Exp::Hint.val());
    let n = s.chars().count() / 2;
    for _ in 0..n {
        print!("{}", prt.next().unwrap());
    }
    println!("{ch:_>widht$}", ch = '_', widht = s.chars().count() - n);
}

// /// shuffling whole deck
// fn shuffle_cards(v: &mut [Cards]) {
//     let mut rng = WyRand::new();
//     rng.shuffle(v);
// }

/// Swap definition and term of deck of cards
pub fn swap_cards(v: &mut [cards::Cards]) {
    for card in v {
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
