use crate::Path;
use colored::Colorize;
use std::fs;

#[derive(Debug)]
pub struct Cards {
    // term in known language
    trm: String,
    // definition in language to be learnt
    def: String,
    // lev: u32,
}

impl Cards {
    fn new(term: &str, def: &str) -> Self {
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
pub fn init(path: &Path, delim: char, n: u8) -> Vec<Cards> {
    let mut tmp_vec: Vec<Cards> = Vec::new();
    let contents = fs::read_to_string(path).expect("what is wronk?");
    // storing lines of contents
    let mut limes = contents.lines();
    for _ in 0..n {
        limes.next();
    }
    // iterating over the lines of file to store them in a vector
    for line in limes {
        let mut words = line.split(delim);

        let trm = words.next().unwrap_or("NO_TERM").trim();
        if trm.is_empty() || trm.chars().next().unwrap() == '#' {
            continue;
        };

        let def = words.next().unwrap_or("NO_DEFINITION").trim();

        let tmp = Cards::new(trm, def);
        tmp_vec.push(tmp);
    }
    println!("{:?} file succesfully readed.", path);
    println!("Basic file looks somehow like this:\n{}", contents);
    for card in &tmp_vec {
        println!("\"{}\":\t\t\t\"{}\"", card.trm.yellow(), card.def.magenta());
    }
    tmp_vec
}

pub fn question(v: Vec<Cards>) -> Vec<Cards> {
    // let mut printer = String::new();
    let mut r = Vec::new();

    for Cards {
        trm: term,
        def: defi,
    } in &v
    {
        println!("\nWord for: {}", term.blue());
        // printer = format!("{printer}\nsay the term for: {}\n", term.blue());
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("hajajajajaja");

        // printer = format!("{printer}{}", guess);
        let guess = guess.trim();
        if guess == defi {
            // printer = format!("{printer}{}\n", "that's about it!".green());
            println!("{}\n", "That's about it!".green());
        } else if guess == "typo" {
            // printer = format!("{printer}\n{}{:?}", "Corrected!".magenta(), r.last());
            println!("{}{:?}", "Corrected: ".magenta(), r.last());
            r.pop();
            if !question(vec![Cards {
                trm: term.to_string(),
                def: defi.to_string(),
            }])
            .is_empty()
            {
                r.push(Cards {
                    trm: term.to_string(),
                    def: defi.to_string(),
                });
            }
        } else if guess == "q" || guess == "quit" || guess == "exit" {
            println!("{}", "exiting...".magenta());
            break;
        } else if guess == "hint" {
            let mut prt = defi.chars();
            print!("Looks like: \"");
            for _ in 0..4 {
                // printer = format!("{printer}{}", prt.next().unwrap());
                print!("{}", prt.next().unwrap());
            }
            println!("{ch:_>widht$}\"", ch = '_', widht = defi.len() - 4);
            if !question(vec![Cards {
                trm: term.to_string(),
                def: defi.to_string(),
            }])
            .is_empty()
            {
                r.push(Cards {
                    trm: term.to_string(),
                    def: defi.to_string(),
                });
            }
        } else {
            r.push(Cards {
                trm: term.to_string(),
                def: defi.to_string(),
            });

            // printer = format!(
            //     "{printer}unfortunately no: {}: {}\nwhile your guess was: \"{}\"\n",
            //     term.yellow(),
            //     defi.blue(),
            //     guess.red()
            // );
            println!(
                "Unfortunately no: {}: {}\nwhile your guess was: \"{}\"\n",
                term.yellow(),
                defi.blue(),
                guess.red()
            );
        }
        println!("{:#>width$}\n\n", "#".magenta(), width = guess.len() + 12);
        // clearscreen::clear().expect("failed to clear screen");
        // println!("{printer}\n\n\n\n");
        // if printer.len() > 2 * 90 {
        //     for _ in 0..6 {
        //         let pos = printer.trim_start().find('\n').unwrap_or(80);
        //         printer.drain(..pos);
        //     }
        // }
    }
    println!("\n\nRemaining cards are {:#?}", r);
    r
}
