use crate::{fs::File, Path};
use colored::Colorize;
use std::{fs, io::Write, process::exit};

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
    fn print_all(&self) {
        println!(
            "{}:\tinf: {}:\tdri: {}\tprä: {}\tperf: {}",
            self.trm.yellow(),
            self.inf.blue(),
            self.dri.magenta(),
            self.pra.green(),
            self.per.cyan()
        );
    }
}

pub fn init(path: &Path, delim: char, n: u8) -> Vec<Verbs> {
    let mut r: Vec<Verbs> = Vec::new();
    // getting contents of file
    let contents = fs::read_to_string(path).expect("what is wronk?");
    // storing lines of contents
    let mut limes = contents.lines();
    // ignoring properties and newline
    for _ in 0..n + 1 {
        limes.next();
    }
    for line in limes {
        let mut words = line.split(delim);

        // lecke
        let no_need = words.next().unwrap_or("NNNNNN").trim();
        if no_need.is_empty() || no_need.starts_with('#') {
            continue;
        };

        let inf = words.next().unwrap_or("").trim();
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

    println!("{:?} file succesfully readed.", path);
    println!("Basic file looks somehow like this:\n{}", contents);
    for card in &r {
        card.print_all();
    }

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
        println!("\n\n\n\nVerbs for: {}", trm.blue());
        // printer = format!("{printer}\nsay the term for: {}\n", term.blue());
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("hajajajajaja");
        let guess = guess.trim();
        if guess == format!("{}, {}, {}, {}", inf, dri, pra, per) {
            println!("{}", "That's about it!".green());
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
            println!("Going to the ones not guessed correctly...");
            break;
        } else if guess == "typo" {
            println!("Removed: {:?}", r.last());
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
            print!("Looks like: \"");
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
        } else if guess == "q" || guess == "quit" || guess == "exit" {
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
            println!("Unfortunately that's not rigth.");
            tmp.print_all();
            r.push(tmp);
            println!("Pushed, will be questioned later on.");
        }
    }
    if !r.is_empty() {
        println!("\n\n{} remaining cards are {:#?}", r.len(), r);
    }

    r
}

pub fn conv(v: &[Verbs], o: &str, delim: char) {
    let mut output = File::create(o).expect("couldn't create file!");
    writeln!(output, "[learnit]").expect("Not succesful.");
    writeln!(output, "[mode: cards]").expect("Not succesful.");
    writeln!(output, "[delim: {delim}]").expect("Not succesful.");
    writeln!(output).expect("Couldn't write to file.");

    let has_header = true;
    if has_header {
    }

    for line in v {
        writeln!(output, "{}{delim}{}", line.trm, line.inf).expect("couldn't write to file!");
    }
    println!("Converting verbs to cards written to {o}");
}
