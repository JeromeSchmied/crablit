use crate::Path;
use colored::Colorize;
use std::fs;

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
    let mut tmp_vec: Vec<Verbs> = Vec::new();
    // getting contents of file
    let contents = fs::read_to_string(path).expect("what is wronk?");
    // storing lines of contents
    let mut limes = contents.lines();
    // ignoring properties and header
    for _ in 0..n + 1 {
        limes.next();
    }
    for line in limes {
        let mut words = line.split(delim);

        // lecke
        let no_need = words.next().unwrap_or("NNNNNN").trim();
        if no_need.is_empty() || no_need.chars().next().unwrap() == '#' {
            continue;
        };

        let inf = words.next().unwrap_or("NO_INFINITIVE").trim();
        let dri = words.next().unwrap_or("NO_THIRD_PERSON").trim();
        let pra = words.next().unwrap_or("NO_PRATERITUM").trim();
        let per = words.next().unwrap_or("NO_PERFEKT").trim();
        let def = words.next().unwrap_or("NO_DEFINITION").trim();
        // other
        let _no_need = words.next().unwrap_or("NNNNNN").trim();

        // making a Verbs of the values
        let tmp = Verbs::new(inf, dri, pra, per, def);
        tmp_vec.push(tmp);
    }

    println!("{:?} file succesfully readed.", path);
    println!("Basic file looks somehow like this:\n{}", contents);
    for card in &tmp_vec {
        card.print_all();
    }

    tmp_vec
}

pub fn question(v: Vec<Verbs>) -> Vec<Verbs> {
    // let mut printer = String::new();
    let mut r = Vec::new();
    for Verbs {
        inf,
        dri,
        pra,
        per,
        trm,
    } in &v
    {
        println!("\n\n\n\nVerbs for: {}", trm.blue());
        // printer = format!("{printer}\nsay the term for: {}\n", term.blue());
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("hajajajajaja");
        let guess = guess.trim();
        if guess == format!("{}, {}, {}, {}", inf, dri, pra, per) {
            println!("{}", "That's about it!".green());
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
            for _ in 0..4 {
                print!("{}", prt.next().unwrap());
            }
            println!("{ch:_>widht$}\"", ch = '_', widht = inf.len() - 4);
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
            break;
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
    println!("\n\nRemaining cards are {:#?}", r);

    r
}
