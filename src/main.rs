extern crate output_vt100;
use clap::Parser;
use colored::Colorize;
use crablit::{verbs::Verbs, *};
use nanorand::{Rng, WyRand};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, author, long_about = None)]
struct Args {
    /// Path of the file to learn
    #[arg(required = true)]
    file: String,

    /// Swap terms and definitions of cards
    #[arg(short, long, default_value_t = false)]
    card_swap: bool,

    /// Sometimes ask the term, sometimes definition of cards
    #[arg(short, long, default_value_t = false)]
    ask_both: bool,

    /// Mode: either cards, verbs or verbs2cards
    #[arg(short, long, default_value = "")]
    mode: String,

    /// Delimiter used in file to seperate terms and definitions
    #[arg(short, long, default_value = "")]
    delim: String,

    /// Shuffle card order
    #[arg(short, long, default_value_t = false)]
    shuffle: bool,
}

fn main() {
    output_vt100::init();
    let args = Args::parse();
    // path recieved as argument
    let path = &args.file;
    dbg!(&path);
    // delimiter as option from console
    let delim_s = &args.delim;
    // let n = 0;
    // dbg!(&n);
    let (mode_det, delim_det, n) = determine_properties(path);
    // mode recieved as argument
    let mode_parsed = &args.mode;

    let mode = if mode_parsed.is_empty() {
        mode_det
    } else {
        Mode::new(mode_parsed)
    };
    dbg!(&mode);
    // delimiter to be used
    // passed as option, same as determined one
    let delim = if !delim_s.is_empty() {
        if delim_det == delim_s.chars().next().unwrap() {
            delim_det
        } else {
            panic!("Delims don't match!");
        }
    } else {
        delim_det
    };
    dbg!(&delim);

    let swap = args.card_swap;
    let both = args.ask_both;
    let p = Path::new(&path);
    match mode {
        Mode::Card => {
            let mut v = init(p, delim, n);
            // println!("Cards:\n\n{:#?}", v);
            // let mut v = cards::init(p, delim, n);
            if swap {
                println!("swapping terms and definitions of each card");
                swap_cards(&mut v);
            }
            if both {
                println!("swapping terms and definitions of some cards");
                random_swap_cards(&mut v);
            }
            while !v.is_empty() {
                let mut rng = WyRand::new();
                if args.shuffle {
                    rng.shuffle(&mut v);
                }
                v = question(v);
            }

            println!("Gone through everything you wanted, great job!");
        }
        Mode::Verb => {
            let mut v: Vec<Verbs> = init(p, delim, n);
            v.remove(0);
            // let mut v = verbs::init(p, delim, n);
            // println!("Verbs:\n\n{:#?}", v);
            println!(
                "\n\n\nStarting to learn verbs, input should be as following: <inf>, <dri>, <prä>, <per>"
            );
            while !v.is_empty() {
                let mut rng = WyRand::new();
                if args.shuffle {
                    rng.shuffle(&mut v);
                }
                // v = verbs::question(v);
                v = question(v);
            }
            println!("Gone through everything you wanted, great job!");
        }
        Mode::VerbConv => {
            let v: Vec<Verbs> = init(p, delim, n);
            // let v = verbs::init(p, delim, n);
            println!(
                "\n\n\nConverting verbs to cards, from file: {:?} to file: {}",
                p,
                "verbs_as_cards.tsv".bright_blue()
            );
            verbs::conv(&v, "verbs_as_cards.tsv", '\t');
        }
    }
}
