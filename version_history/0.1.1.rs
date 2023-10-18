use colored::Colorize;
use rand::{seq::SliceRandom, thread_rng};

use std::fs;
// use std::fs::read_to_string;
// use std::io::Read;
use std::path::Path;

#[derive(Debug)]
struct Cards {
    // term in known language
    trm: String,
    // definition in language to be learnt
    def: String,
    // lev: u32,
}

// impl Cards {
//     fn new(term: &str, def: &str) -> Self {
//         Self {
//             trm: term.to_string(),
//             def: def.to_string(),
//         }
//     }
// }

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

fn main() {
    // hardcoded();
    let fname = user_input("What's the filename containing flashcards to be learnt?");
    // let delim = user_input("What character is the delimiter?");
    // let v = init(&fname, delim.chars().nth(0).unwrap_or(';'));
    let p = Path::new(&fname);
    // let mut v = init(p, delim.chars().nth(0).unwrap_or(';'));
    let mut v = init(p);
    // v.shuffle(&mut thread_rng());
    while !v.is_empty() {
        v.shuffle(&mut thread_rng());
        v = question(v);
    }
}

fn user_input(qst: &str) -> String {
    println!("{qst}");
    let mut babineni = String::new();
    std::io::stdin()
        .read_line(&mut babineni)
        .expect("what is goink on?");
    // let babineni = babineni.pop().unwrap_or('\t').to_string();
    if babineni.ends_with('\n') {
        babineni.pop();
    }
    // let babineni = babineni;
    babineni
}

#[allow(unused)]
fn init(fname: &Path) -> Vec<Cards> {
    let mut tmp_vec: Vec<Cards> = Vec::new();
    println!("Trying to open {:?}", fname);
    let delim;
    // let lang;
    // let creator;

    // getting contents of file
    let contents = fs::read_to_string(fname).expect("what is wronk?");
    // storin lines of contents
    let mut limes = contents.lines();
    // seeing wether first line includes [learnit] to know if it is made for learnit
    if limes.next().unwrap() == "[learnit]" {
        // getting delimiter from next line as string and converting it into a character
        delim = limes.next().unwrap_or(";").chars().nth(1).unwrap_or(';');
        // lang = limes.next().unwrap_or("unknown");
        // creator = limes.next().unwrap_or("anonymus");
    } else {
        // lang = "unknown";
        // creator = "anonymus";
        // resetting lines to start from beginning
        limes = contents.lines();
        // asking for user input as delimiter is unknown
        delim = user_input("What character is the delimiter?")
            .chars()
            .next()
            .unwrap_or(';');
    }

    // iterating over the lines of file to store them in a vector
    for line in /* fs::read_to_string(fname).expect("bajos fajl!\n").lines() */ limes {
        let mut words = line.split(delim);
        let trm = words.next().unwrap_or("NANANA").trim();
        let def = words.next().unwrap_or("NANANA").trim();
        let tmp = Cards {
            trm: trm.to_string(),
            def: def.to_string(),
        };
        tmp_vec.push(tmp);
    }
    println!("{:?} file succesfully readed.", fname);
    println!("Basic file looks somehow like this:\n{}", contents);
    for Cards { trm, def } in &tmp_vec {
        // let num = def.len() + 20 - trm.len();
        // println!("{}:{def:>num$}", trm.yellow());
        println!("\"{}\":\t\t\t\"{}\"", trm.yellow(), def.magenta());
    }
    tmp_vec
}

fn question(v: Vec<Cards>) -> Vec<Cards> {
    let mut printer = String::new();
    let mut r = Vec::new();
    'manus: for Cards {
        trm: term,
        def: defi,
    } in &v
    {
        println!("\nsay the term for: {}", term.blue());
        printer = format!("{printer}\nsay the term for: {}\n", term.blue());
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("hajajajajaja");

        printer = format!("{printer}{}", guess);
        let guess = guess.trim();
        // println!("your guess: {}", guess.blue());
        if guess == defi {
            // println!("{}", "that's about it!".green());
            printer = format!("{printer}{}\n", "that's about it!".green());
        } else if guess == "q" || guess == "quit" || guess == "exit" {
            println!("{}", "exiting...".magenta());
            break 'manus;
        // } else if guess == "typo" {
        //     r.pop();
        } else if guess == "hint" {
            // println!("{:?}{num:|>num$}", defi.chars(), num = defi.len() - 4);
            let mut prt = defi.chars();
            for _ in 0..4 {
                printer = format!("{printer}{}", prt.next().unwrap());
            }
            r.push(Cards {
                trm: term.to_string(),
                def: defi.to_string(),
            });
        } else {
            r.push(Cards {
                trm: term.to_string(),
                def: defi.to_string(),
            });

            // println!(
            //     "unfortunately no: {}: {}\nwhile your guess was: \"{}\"",
            //     term.yellow(),
            //     defi.blue(),
            //     guess.red()
            // );
            printer = format!(
                "{printer}unfortunately no: {}: {}\nwhile your guess was: \"{}\"\n",
                term.yellow(),
                defi.blue(),
                guess.red()
            );
        }
        clearscreen::clear().expect("failed to clear screen");
        println!("{printer}\n\n\n\n");
        // print!("{:>8}", "|".green());
        // printer.remove(0);
        if printer.len() > 2 * 90 {
            // let pos = printer.split('\n').nth(4).unwrap();
            for _ in 0..6 {
                let pos = printer.trim_start().find('\n').unwrap_or(80);
                // println!("first newline: {pos}");
                printer.drain(..pos);
            }
        }
    }
    // println!("\n\nAll cards are: {:#?}", v);
    println!("\n\nRemaining cards are {:#?}", r);
    r
}
