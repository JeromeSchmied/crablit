use clap::Parser;
use colored::Colorize;
use crablit::*;
use nanorand::{Rng, WyRand};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, author, long_about = None)]
struct Args {
    /// Path of the file to learn.
    #[arg(required = true)]
    file: String,

    // /// Controls whether to color the output or not.
    // #[arg(short, long, default_value_t = true)]
    // color: bool,
    /// Mode, either cards, verbs or verbs2cards, useful when want to convert from verbs to cards, or when using verbs.
    #[arg(short, long, default_value = "cards")]
    mode: String,

    /// Delimiter used in file to seperate terms and definitions.
    #[arg(short, long, default_value = "")]
    delim: String,

    /// Swap terms and definitions. Only if mode is cards.
    #[arg(short, long, default_value_t = false)]
    swap: bool,

    /// Sometimes ask for term, sometimes definition. Only if mode is cards.
    #[arg(short, long, default_value_t = false)]
    both: bool,
    // /// Has header starting with [crablit] or not.
    // #[arg(short, long, default_value_t = false)]
    // header: bool,
    // /// The number of lines to skip
}

fn main() {
    // path recieved as argument
    let path = Args::parse().file;
    dbg!(&path);
    // mode recieved as argument
    let mode = Mode::new(&Args::parse().mode);
    dbg!(&mode);
    // delimiter as option from console
    let delim_s = Args::parse().delim;
    // let n = 0;
    // dbg!(&n);
    let (_mode_det, delim_det, n) = determine_properties(&path);
    // delimiter to be used
    let delim;
    // passed as option, same as determined one
    if !delim_s.is_empty() {
        if delim_det == delim_s.chars().next().unwrap() {
            delim = delim_det;
        } else {
            panic!("Delims don't match!");
        }
    } else {
        delim = delim_det;
    }
    dbg!(&delim);

    let swap = Args::parse().swap;
    let both = Args::parse().both;
    let p = Path::new(&path);
    match mode {
        Mode::Card => {
            let mut v = cards::init(p, delim, n);
            if swap {
                swap_cards(&mut v);
            }
            if both {
                random_swap_cards(&mut v);
            }
            while !v.is_empty() {
                let mut rng = WyRand::new();
                rng.shuffle(&mut v);
                v = cards::question(v);
            }
            println!("Gone through everything you wanted, great job!");
        }
        Mode::Verb => {
            let mut v = verbs::init(p, delim, n);
            println!(
                "\n\n\nStarting to learn verbs, input should be as following: <inf>, <dri>, <prä>, <per>\n\n\n"
            );
            while !v.is_empty() {
                let mut rng = WyRand::new();
                rng.shuffle(&mut v);
                v = verbs::question(v);
            }
            println!("Gone through everything you wanted, great job!");
        }
        Mode::VerbConv => {
            let v = verbs::init(p, delim, n);
            println!(
                "\n\n\nConverting verbs to cards, from file: {:?} to file: {}",
                p,
                "verbs_as_cards.tsv".bright_blue()
            );
            verbs::conv(&v, "verbs_as_cards.tsv", '\t');
        }
    }

    // let (delim, p) = nice_args(args);
    // let mut v = cards::init(Path::new("big.txt"), ';', 4);
    // while !v.is_empty() {
    //     v.shuffle(&mut thread_rng());
    //     v = cards::question(v);
    // }
}
