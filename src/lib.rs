use rand::{seq::SliceRandom, thread_rng};
use std::path::Path;
use std::{env, fs};

pub mod cards;
pub mod verbs;

pub enum Game {
    Card,
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
    let file_path = match args.get(1) {
        Some(f) => f.trim().to_owned(),
        None => {
            let fp = user_input("File path: ");
            fp
        }
    };

    if file_path == "-h" || file_path == "--help" {
        // println!("Docs coming soon...");
        println!(
            r#"
         A program to learn words in the terminal.

         Usage:
           flashcards [options] [file]        Learn file

         Options:
            -h || --help: show this message"#
        );
        return;
    }

    // let p = Path::new(&file_path);
    let p = &file_path.to_owned();
    // let mut p = String::from(&file_path);
    let (mode, delim, n, path) = determine_properties(p);

    let p = Path::new(&path);
    match mode {
        Game::Card => {
            let mut v = cards::init(p, delim, n);
            while !v.is_empty() {
                v.shuffle(&mut thread_rng());
                v = cards::question(v);
            }
        }
        Game::Verb => {
            let mut v = verbs::init(p, delim, n);
            while !v.is_empty() {
                v.shuffle(&mut thread_rng());
                v = verbs::question(v);
            }
        }
        Game::Bad => println!("Something unexpected happened, exiting..."),
        Game::Help => println!("Docs coming soon..."),
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

pub fn determine_properties(path: &String) -> (Game, char, u8, String) {
    println!("Trying to open {:?}", path);
    let mut contents = fs::read_to_string(&path).unwrap_or("redo".to_string());
    let mut path_fixed = path.to_owned();
    loop {
        if contents == "redo" {
            println!("No luck...");
            path_fixed = user_input("Sorry, what is the correct path?");
            contents = fs::read_to_string(&path_fixed).unwrap_or("redo".to_string());
        } else {
            break;
        }
        println!("Trying to open {:?}", path_fixed);
    }
    let delim: char;
    let mode: String;
    // plus one for [learnit], one for an extra newline at the end
    let mut num = 2;
    // let lang;
    // let creator;

    // getting contents of file
    // storing lines of contents

    let mut limes = contents.lines();
    // checking wether first line includes [learnit] to know if it is made for learnit
    if limes.next().unwrap() == "[learnit]" {
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
    if mode == "[mode: verbs]" || mode == "verbs" {
        (Game::Verb, delim, num, path_fixed)
    } else if mode == "[mode: cards]" || mode == "cards" {
        (Game::Card, delim, num, path_fixed)
    } else {
        (Game::Bad, delim, num, path_fixed)
    }
}
