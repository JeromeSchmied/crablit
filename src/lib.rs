use colored::Colorize;
use rand::{seq::SliceRandom, thread_rng};
use std::path::Path;
use std::process::exit;
use std::{env, fs};

pub mod cards;
pub mod verbs;

pub enum Type {
    Card,
    VerbConv,
    Verb,
    Help,
    Bad,
}

pub fn start() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    // let mode = args.get(1).unwrap_or("");
    // let mode: String = match args.get(1) {
    //     Some(m) => m.to_owned(),
    //     None => {
    //         let m = user_input("Mode(verb,basic): ");
    //         m
    //     }
    // };
    // if args.iter().find(|&x| x == "help").unwrap_or(&"-h") == "--help" {show_help()}
    // if args.iter().find(|&x| x == "conv").is_some() {}

    let file_path = match args.last() {
        Some(f) => f.trim().to_owned(),
        None => user_input("File path: "),
    };
    if file_path == "-h" || file_path == "--help" {
        show_help();
    }
    // let p = Path::new(&file_path);
    let p = &file_path.to_owned();
    // let mut p = String::from(&file_path);
    let (mode, delim, n, path) = determine_properties(p);

    let p = Path::new(&path);
    match mode {
        Type::Card => {
            let mut v = cards::init(p, delim, n);
            while !v.is_empty() {
                v.shuffle(&mut thread_rng());
                v = cards::question(v);
            }
            println!("Gone through everything you wanted, great job!");
        }
        Type::Verb => {
            let mut v = verbs::init(p, delim, n);
            println!(
                "\n\n\nStarting to learn verbs, input should be as following: <inf>, <dri>, <prä>, <per>\n\n\n"
            );
            while !v.is_empty() {
                v.shuffle(&mut thread_rng());
                v = verbs::question(v);
            }
            println!("Gone through everything you wanted, great job!");
        }
        Type::VerbConv => {
            let v = verbs::init(p, delim, n);
            println!(
                "\n\n\nConverting verbs to cards, from file: {:?} to file: {}",
                p, "verbs_as_cards.tsv"
            );
            verbs::conv(&v, "verbs_as_cards.tsv", '\t');
        }
        Type::Bad => println!("Something unexpected happened, exiting..."),
        Type::Help => println!("Docs coming soon..."),
    }

    // if mode == "verb" || mode == "verben" || mode == "verbs" {
    //     let mut v = verbs::init(p, delim);
    //     while !v.is_empty() {
    //         v.shuffle(&mut thread_rng());
    //         v = verbs::question(v);
    //     }
    // } else if mode == "basic" || mode == "flashcards" || mode == "cards" || mode.is_empty() {
    //     let mut v = basic::init(p);
    //     while !v.is_empty() {
    //         v.shuffle(&mut thread_rng());
    //         v = basic::question(v);
    //     }
    // } else {
    //     println!("unfortunately no mode found.\nquitting...");
    // }
}

pub fn user_input(qst: &str) -> String {
    println!("{qst}");
    let mut babineni = String::new();
    std::io::stdin()
        .read_line(&mut babineni)
        .expect("what is goink on?");
    if babineni.ends_with('\n') {
        babineni.pop();
    }
    babineni
}

pub fn determine_properties(path: &String) -> (Type, char, u8, String) {
    println!("Trying to open {:?}", path);
    let mut contents = fs::read_to_string(path).unwrap_or("redo".to_string());
    let mut path_fixed = path.to_owned();
    loop {
        if contents == "redo" {
            println!("No luck opening the file...");
            path_fixed = user_input("Sorry, what is the correct path?");
            contents = fs::read_to_string(&path_fixed).unwrap_or("redo".to_string());
        } else {
            break;
        }
        println!("Trying to open {:?}", path_fixed);
    }
    let delim: char;
    let mode: String;
    // plus one for [crablit], one for an extra newline at the end
    let mut num = 2;
    // let lang;
    // let creator;

    // getting contents of file
    // storing lines of contents

    let mut limes = contents.lines();
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
        // lang = limes.next().unwrap_or("unknown");
        // creator = limes.next().unwrap_or("anonymus");
    } else {
        // lang = "unknown";
        // creator = "anonymus";
        // resetting lines to start from beginning
        // asking for user input as delimiter is unknown
        mode = user_input("Mode(cards/verbs)?").to_string();
        delim = user_input("What character is the delimiter?")
            .chars()
            .next()
            .unwrap_or(';');
        num = 0;
    }
    println!(
        "Mode: \"{}\", delimiter: \"{}\", number of lines skipping: \"{}\"",
        mode, delim, num
    );
    if mode == "[mode: verbs]" || mode == "verbs" || mode == "[verbs]" || mode == "[mode: verb]" {
        (Type::Verb, delim, num, path_fixed)
    } else if mode == "[mode: cards]" || mode == "cards" || mode == "[cards]" {
        (Type::Card, delim, num, path_fixed)
    } else if mode == "[mode: conv]" || mode == "conv" || mode == "verb_conv" {
        (Type::VerbConv, delim, num, path_fixed)
    } else {
        (Type::Bad, delim, num, path_fixed)
    }
}

fn show_help() {
    // println!("Docs coming soon...");
    println!("A program to learn words in the terminal.");
    println!();
    println!("{}", "Usage:".underline().bold());
    println!("  crablit [options] file      Learn file");
    println!("{}", "Options:".underline().bold());
    println!("  -h, --help: show this message.");
    //         println!(
    //             r#"
    // A program to learn words in the terminal.
    //
    // Usage:
    //   flashcards [options] [file]        Learn file
    //
    // Options:
    //   -h || --help: show this message"#
    //         );
    exit(0);
}
