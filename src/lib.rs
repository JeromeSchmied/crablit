use colored::Colorize;
use nanorand::{Rng, WyRand};
use rustyline::DefaultEditor;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    process::exit,
};

// mod args;
mod cards;
mod verbs;

enum Type {
    Card,
    VerbConv,
    Verb,
    Bad,
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

pub fn start(args: &[String]) {
    // dbg!(&args);
    // if args.iter().find(|&x| x == "conv").is_some() {}
    // let mut p = String::from(&file_path);
    let (mode, delim, n, path) = determine_properties(args);

    let p = Path::new(&path);
    match mode {
        Type::Card => {
            let mut v = cards::init(p, delim, n);
            while !v.is_empty() {
                // fastrand::shuffle(&mut v);
                // v.shuffle(&mut thread_rng());
                let mut rng = WyRand::new();
                rng.shuffle(&mut v);
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
                // v.shuffle(&mut thread_rng());
                // fastrand::shuffle(&mut v);
                let mut rng = WyRand::new();
                rng.shuffle(&mut v);
                v = verbs::question(v);
            }
            println!("Gone through everything you wanted, great job!");
        }
        Type::VerbConv => {
            let v = verbs::init(p, delim, n);
            println!(
                "\n\n\nConverting verbs to cards, from file: {:?} to file: {}",
                p,
                "verbs_as_cards.tsv".bright_blue()
            );
            verbs::conv(&v, "verbs_as_cards.tsv", '\t');
        }
        Type::Bad => println!("Something unexpected happened, exiting..."),
    }
}

fn user_input(qst: &str) -> String {
    println!("{qst}");
    let mut babineni;

    let mut rl = DefaultEditor::new().expect("couldnt read");
    babineni = rl.readline("").expect("Couldnt read");
    // std::io::stdin()
    //     .read_line(&mut babineni)
    //     .expect("what is goink on?");
    if babineni.ends_with('\n') {
        babineni.pop();
    }
    babineni
}

fn determine_properties(args: &[String]) -> (Type, char, u8, String) {
    if args.len() > 1 {
        if args.iter().any(|x| x == "-h" || x == "--help") {
            show_help()
        }
    // if nothing was specified, quit
    } else {
        println!("{}", "File was not specified:".red());
        show_help();
    }

    let path = match args.last() {
        Some(f) => f.trim().to_owned(),
        None => user_input("File path: "),
    };
    // let path = &file_path.to_owned();

    println!("Trying to open {:?}", &path);
    let f = File::open(&path).expect("couldn't open file");
    let mut br = BufReader::new(f);
    let mut limes = String::new();
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

    // getting contents of file
    // storing lines of contents

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
        // lang = limes.next().unwrap_or("unknown");
        // creator = limes.next().unwrap_or("anonymus");
    } else {
        // lang = "unknown";
        // creator = "anonymus";
        // asking for user input as delimiter is unknown
        // if limes.next().unwrap_or("".to_owned()).find() {}
        // if get_delim(limes.last()) {
        let last = limes.clone().last().unwrap_or("");
        loop {
            let line = &limes.next().unwrap_or("");
            if !(line.is_empty() || line.starts_with('#')) && get_delim(line) == get_delim(last) {
                delim = get_delim(line);
                break;
            }
        }
        // delim = get_delim(limes.last());
        // } else {
        //     delim = ';';
        // }
        // mode = user_input("Mode(cards/verbs)?").to_string();
        mode = "cards".to_owned();
        // delim = user_input("What.to_owned() character is the delimiter?")
        //     .chars()
        //     .next()
        //     .unwrap_or(';');
        // resetting lines to start from beginning
        num = 0;
    }
    println!(
        "Mode: \"{}\", delimiter: \"{}\", number of lines skipping: \"{}\"",
        mode, delim, num
    );
    if mode == "[mode: verbs]" || mode == "verbs" || mode == "[verbs]" || mode == "[mode: verb]" {
        (Type::Verb, delim, num, path)
    } else if mode == "[mode: cards]" || mode == "cards" || mode == "[cards]" {
        (Type::Card, delim, num, path)
    } else if mode == "[mode: conv]" || mode == "conv" || mode == "verb_conv" {
        (Type::VerbConv, delim, num, path)
    } else {
        (Type::Bad, delim, num, path)
    }
}

fn show_help() {
    println!("A vocabulary learning program for the terminal.");
    println!();
    println!("{}", "Usage:".underline().bold());
    println!("  crablit [options] file        Learn file");
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

fn get_delim(line: &str) -> char {
    const DELIMS: [char; 6] = [';', '|', '\t', ':', ',', '-'];
    // let line = line.unwrap_or("".to_owned());
    for delim in DELIMS {
        if !(line.is_empty() || line.starts_with('#')) && line.chars().any(|x| x == delim) {
            return delim;
        }
    }
    user_input("What character is the delimiter?")
        .chars()
        .next()
        .unwrap_or(';')
}
