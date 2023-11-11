use crate::BufReader;
use crate::File;
use colored::Colorize;
use rustyline::DefaultEditor;
use std::io::BufRead;
use std::path::Path;
use std::{io::Write, process::exit};

#[derive(Debug)]
pub struct Verbs {
    inf: String,
    dri: String,
    pra: String,
    per: String,
    trm: String,
}

impl Verbs {
    fn new(inf: &str, dri: &str, pra: &str, per: &str, def: &str) -> Self {
        Verbs {
            inf: inf.to_owned(),
            dri: dri.to_owned(),
            pra: pra.to_owned(),
            per: per.to_owned(),
            trm: def.to_owned(),
        }
    }
    // fn print_all(&self) {
    //     println!(
    //         "{}:\tinf: {}:\tdri: {}\tprä: {}\tperf: {}",
    //         self.trm.yellow(),
    //         self.inf.blue(),
    //         self.dri.magenta(),
    //         self.pra.green(),
    //         self.per.cyan()
    //     );
    // }
    fn print_em(&self) {
        print!(
            "{}, {}, {}, {}",
            self.inf.yellow(),
            self.dri.blue(),
            self.pra.cyan(),
            self.per.magenta()
        );
    }
}

pub fn init(path: &Path, delim: char, n: u8) -> Vec<Verbs> {
    let mut r: Vec<Verbs> = Vec::new();
    // getting contents of file

    let br = BufReader::new(File::open(path).expect("Couldn't open file, quitting..."));
    // storing lines of contents
    let mut lines = br.lines();
    // ignoring properties and newline
    for _ in 0..n + 1 {
        lines.next();
    }
    for line in lines {
        let line = line.expect("Something wrong with bufread line");
        let mut words = line.split(delim);

        let inf = words.next().unwrap_or("").trim();
        if inf.is_empty() || inf.starts_with('#') {
            continue;
        };

        let dri = words.next().unwrap_or("").trim();
        let pra = words.next().unwrap_or("").trim();
        let per = words.next().unwrap_or("").trim();
        let def = words.next().unwrap_or("").trim();
        // other
        let _no_need = words.next().unwrap_or("NNNNNN").trim();

        // making a Verbs of the values
        let tmp = Verbs::new(inf, dri, pra, per, def);
        r.push(tmp);
    }
    // deleting header
    r.remove(0);

    println!("{:?} file succesfully read.", path);
    // println!("Basic file looks somehow like this:\n{}", contents);
    // for card in &r {
    //     card.print_all();
    // }

    r
}

pub fn question(v: Vec<Verbs>) -> Vec<Verbs> {
    // let mut printer = String::new();
    if v.len() != 1 {
        println!("\n\nYou have {} verbs to learn, let's start!", v.len());
    }
    // results
    let mut r = Vec::new();
    for Verbs {
        inf,
        dri,
        pra,
        per,
        trm,
    } in &v
    {
        if inf.is_empty() {
            println!("Oh, no! Missing verbform found!");
            continue;
        }

        println!("\n\n\n? {}", trm.blue());
        // printer = format!("{printer}\nsay the term for: {}\n", term.blue());
        let guess;
        let mut rl = DefaultEditor::new().expect("Something is wronk...");
        guess = rl.readline("> ").expect("Well");
        // std::io::stdin()
        //     .read_line(&mut guess)
        //     .expect("hajajajajaja");
        let guess = guess.trim();
        if guess == format!("{}, {}, {}, {}", inf, dri, pra, per) {
            println!("{}\n", "% That's about it!".green());
        } else if guess == "skip" {
            println!(
                "skipping: {:?}",
                Verbs {
                    inf: inf.to_string(),
                    dri: dri.to_string(),
                    pra: pra.to_string(),
                    per: per.to_string(),
                    trm: trm.to_string(),
                }
            );
            continue;
        } else if guess == "revise" {
            if r.len() == 1 {
                println!("Type revise again!");
            } else if r.is_empty() {
                println!("Nothing to revise, you might to type it again to make it work...");
            } else {
                println!("Going to the ones not guessed correctly...");
            }
            break;
        } else if guess == "typo" {
            println!("{} {:?}", "Removed:".magenta(), r.last());
            r.pop();
            if !question(vec![Verbs {
                inf: inf.to_string(),
                dri: dri.to_string(),
                pra: pra.to_string(),
                per: per.to_string(),
                trm: trm.to_string(),
            }])
            .is_empty()
            {
                r.push(Verbs {
                    inf: inf.to_owned(),
                    dri: dri.to_owned(),
                    pra: pra.to_owned(),
                    per: per.to_owned(),
                    trm: trm.to_owned(),
                })
            }
        } else if guess == "hint" {
            let mut prt = inf.chars();
            print!("{} \"", "#".cyan());
            let n = inf.len() / 2;
            for _ in 0..n {
                print!(
                    "{}",
                    prt.next().expect("Hint couldn't find next character!")
                );
            }
            // println!("\"");
            println!("{ch:_>widht$}\"", ch = '_', widht = inf.len() - n);
            if !question(vec![Verbs {
                inf: inf.to_string(),
                dri: dri.to_string(),
                pra: pra.to_string(),
                per: per.to_string(),
                trm: trm.to_string(),
            }])
            .is_empty()
            {
                r.push(Verbs {
                    inf: inf.to_owned(),
                    dri: dri.to_owned(),
                    pra: pra.to_owned(),
                    per: per.to_owned(),
                    trm: trm.to_owned(),
                })
            }
        } else if guess == ":q" || guess == "quit" || guess == "exit" {
            println!("{}", "exiting...".magenta());
            // break; <- doesn't work, as the outer while keeps repeating
            exit(0);
        } else {
            let tmp = Verbs {
                inf: inf.to_owned(),
                dri: dri.to_owned(),
                pra: pra.to_owned(),
                per: per.to_owned(),
                trm: trm.to_owned(),
            };
            // println!("Unfortunately that's not rigth.");
            // tmp.print_all();
            print!("{} ", "~".bright_red());
            tmp.print_em();
            println!(" <- {}", "was the right answer.".red());
            r.push(tmp);
            // println!("{}", "Pushed, will be questioned later on.".magenta());
        }
    }
    if r.len() > 1 {
        println!("\n\n{} remaining cards are {:#?}", r.len(), r);
    }

    r
}

pub fn conv(v: &[Verbs], o: &str, delim: char) {
    let mut output = File::create(o).expect("couldn't create file!");
    writeln!(output, "[crablit]").expect("Not succesful.");
    writeln!(output, "[mode: cards]").expect("Not succesful.");
    writeln!(output, "[delim: {delim}]").expect("Not succesful.");
    writeln!(output).expect("Couldn't write to file.");

    // let has_header = true;
    // if has_header {}

    for line in v {
        writeln!(output, "{}{delim}{}", line.trm, line.inf).expect("couldn't write to file!");
    }
    println!("Converting verbs to cards written to {o}");
}
