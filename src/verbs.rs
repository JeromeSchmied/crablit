use crate::flashcards::user_input;
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

pub fn init(path: &Path) -> Vec<Verbs> {
    let mut tmp_vec: Vec<Verbs> = Vec::new();
    println!("Trying to open {:?}", path);
    let delim;
    // let lang;
    // let creator;

    // getting contents of file
    let contents = fs::read_to_string(path).expect("what is wronk?");
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
    for line in limes {
        let mut words = line.split(delim);

        // lecke
        let _no_need = words.next().unwrap_or("NNNNNN").trim();

        let inf = words.next().unwrap_or("NANANA").trim();
        let dri = words.next().unwrap_or("NANANA").trim();
        let pra = words.next().unwrap_or("NANANA").trim();
        let per = words.next().unwrap_or("NANANA").trim();
        let def = words.next().unwrap_or("NANANA").trim();
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
        println!("\n\n\n\nsay the stuff for: {}", trm.blue());
        // printer = format!("{printer}\nsay the term for: {}\n", term.blue());
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("hajajajajaja");
        let guess = guess.trim();
        if guess == format!("{}, {}, {}, {}", inf, dri, pra, per) {
            println!("{}", "That's about it!".green());
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
