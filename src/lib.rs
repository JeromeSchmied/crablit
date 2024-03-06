//! # Library for vocabulary learning, used in `crablit`.
use crate::utils::{exit, knew, revise, togo, typo, SPACER};
use owo_colors::OwoColorize;
use rustyline::DefaultEditor;
use std::{
    error::Error,
    fmt::Debug,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process,
};

/// Module for learning Deck of Cards
pub mod cards;
/// Module for parsing cli arguments
pub mod config;
/// Module for saving state: progress
pub mod state;
/// enums, messages
pub mod utils;

// re-exports
pub use cards::Card;
pub use utils::Lok;
// pub use verbs::Verb;

// enum Kard {
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
pub fn init(path: &PathBuf, delim: char) -> Result<Vec<Card>, Box<dyn Error>> {
    // contents of file with vocab data
    let contents = fs::read_to_string(path)?;
    // results vector
    let mut r = Vec::new();
    // iterating over the lines of file to store them in a vector
    for line in contents.lines() {
        // if is comment or empty
        if line.trim().starts_with('#') || line.trim().is_empty() {
            continue;
        }
        r.push(Card::deser(line, delim)?);
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
pub fn question(v: &mut [Card], conf: &config::Config) -> Result<(), Box<dyn Error>> {
    // let mut printer = String::new();
    let len = v.iter().filter(|item| item.lok() != Lok::Done).count();
    println!("\n\nYou have {len} words to learn, let's start!\n\n");
    let mut rl = DefaultEditor::new()?;

    let mut i = 0;
    let mut prev_valid_i: i32;
    while i < v.len() {
        let item = &mut v[i];

        if item.lok() == Lok::Done {
            i += 1;
            continue;
        }
        prev_valid_i = i as i32 - 1;
        // display prompt
        let last_hr = rl.history().iter().last();
        // eprintln!("last history element: {:?}", last_hr);
        let msg = format!(
            "{}{SPACER}> ",
            if last_hr.is_some_and(|he| {
                he.starts_with(":h") || he == ":typo" || he == ":n" || he == ":num" || he == ":togo"
            }) {
                "".to_string()
            } else {
                format!("{}\n", item.question())
            }
        );

        let guess = rl.readline(&msg)?;
        rl.add_history_entry(&guess)?;
        let guess = guess.trim();

        // is command
        if guess.starts_with(':') {
            match guess {
                ":q" | ":quit" | ":exit" => {
                    println!("{}", exit());
                    process::exit(0);
                }

                ":h" | ":help" | ":hint" => {
                    println!("{}", item.hint());
                }

                ":w" | ":write" | ":save" => {
                    state::save_prog(v, conf)?;
                }

                ":wq" => {
                    state::save_prog(v, conf)?;
                    println!("{}", exit());
                    process::exit(0);
                }

                ":typo" => {
                    // ask to type again before correcting?
                    if i > 0 {
                        if let Some(skipping) = v.get(prev_valid_i as usize) {
                            println!("{}", typo(&skipping.ser(" = ")));
                            v[prev_valid_i as usize].incr();
                        } else {
                            println!("{}", typo("None"));
                        }
                    } else {
                        println!("{}", typo("None"));
                    }
                    // rl.readline(&msg)?;
                }

                ":skip" => {
                    println!("{}\n\n", item.skip());
                    i += 1;
                    continue;
                }

                ":revise" => {
                    println!("{}", revise());
                    break;
                }

                ":f" | ":flash" => {
                    println!("{}\n\n\n", item.flashcard());
                    item.incr();
                    i += 1;
                }

                // incorrect, not accurate
                ":n" | ":num" | ":togo" => {
                    println!("{}", togo(len, (prev_valid_i + 1).try_into()?));
                }

                uc => {
                    println!("{} {}\n", "Unknown command:".red(), uc);
                }
            }
        } else if guess == item.correct() {
            println!("{}\n", knew());
            item.incr();
            i += 1;
        } else {
            println!("{}", item.wrong());
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
/// - `init()`
/// - `question()`
/// - `state::rm()`
/// - `verbs::deser_to_card()`
pub fn run(conf: &config::Config) -> Result<(), Box<dyn Error>> {
    match conf.convert() {
        false => {
            let mut v = init(&conf.file_path(), conf.delim())?;
            if conf.swap() {
                println!("swapping terms and definitions of each card");
                cards::swap(&mut v);
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
        true => {
            let v = init(&conf.file_path(), conf.delim())?;
            let data = cards::deser_verbs_to_cards(&v, conf)?;

            let pb = PathBuf::from(&conf.file_path_orig());
            let outf_name = format!("{}_as_cards.csv", pb.file_stem().unwrap().to_str().unwrap());
            println!(
                "\n\nConverting verbs to cards, from file: {:?} to file: {}",
                conf.file_path_orig(),
                outf_name.bright_blue()
            );
            let mut out_f = File::create(outf_name)?;

            writeln!(out_f, "# [crablit]")?;
            writeln!(out_f, "# mode = \"cards\"")?;
            writeln!(out_f, "# delim = \'{}\'\n\n", conf.delim())?;
            writeln!(out_f, "{data}")?;

            println!("Converting from verbs to cards done");

            Ok(())
        }
    }
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
            card.swap_me();
        }
    }
}
