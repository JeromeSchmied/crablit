use crate::flashcards::user_input;
use crate::Path;
use colored::Colorize;
use std::fs;

#[derive(Debug)]
pub struct Cards {
    // term in known language
    pub trm: String,
    // definition in language to be learnt
    pub def: String,
    // lev: u32,
}

impl Cards {
    pub fn new(term: &str, def: &str) -> Self {
        Self {
            trm: term.to_string(),
            def: def.to_string(),
        }
    }
    // pub fn get_trm(self) -> String {
    //     self.trm
    // }
    // pub fn get_def(self) -> String {
    //     self.def
    // }
}

#[allow(unused)]
pub fn init(fname: &Path) -> Vec<Cards> {
    let mut tmp_vec: Vec<Cards> = Vec::new();
    println!("Trying to open {:?}", fname);
    let delim;
    // let lang;
    // let creator;

    // getting contents of file
    let contents = fs::read_to_string(fname).expect("what is wronk?");
    // storing lines of contents
    let mut limes = contents.lines();
    // checking wether first line includes [learnit] to know if it is made for learnit
    if limes.next().unwrap() == "[learnit]" {
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
    for line in limes {
        let mut words = line.split(delim);
        let trm = words.next().unwrap_or("NANANA").trim();
        let def = words.next().unwrap_or("NANANA").trim();

        let tmp = Cards::new(trm, def);
        tmp_vec.push(tmp);
    }
    println!("{:?} file succesfully readed.", fname);
    println!("Basic file looks somehow like this:\n{}", contents);
    for card in &tmp_vec {
        println!("\"{}\":\t\t\t\"{}\"", card.trm.yellow(), card.def.magenta());
    }
    tmp_vec
}

pub fn question(v: Vec<Cards>) -> Vec<Cards> {
    let mut printer = String::new();
    let mut r = Vec::new();
    'manus: for Cards {
        trm: term,
        def: defi,
    } in &v
    {
        println!("\nsay the definition for: {}", term.blue());
        printer = format!("{printer}\nsay the term for: {}\n", term.blue());
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("hajajajajaja");

        printer = format!("{printer}{}", guess);
        let guess = guess.trim();
        if guess == defi {
            printer = format!("{printer}{}\n", "that's about it!".green());
        } else if guess == "q" || guess == "quit" || guess == "exit" {
            println!("{}", "exiting...".magenta());
            break 'manus;
        } else if guess == "hint" {
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

            printer = format!(
                "{printer}unfortunately no: {}: {}\nwhile your guess was: \"{}\"\n",
                term.yellow(),
                defi.blue(),
                guess.red()
            );
        }
        clearscreen::clear().expect("failed to clear screen");
        println!("{printer}\n\n\n\n");
        if printer.len() > 2 * 90 {
            for _ in 0..6 {
                let pos = printer.trim_start().find('\n').unwrap_or(80);
                printer.drain(..pos);
            }
        }
    }
    println!("\n\nRemaining cards are {:#?}", r);
    r
}
