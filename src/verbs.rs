use crate::{user_input, BufReader, Colorize, Exp, File, Path};
use std::{
    io::{BufRead, Write},
    process::exit,
};

#[derive(Debug)]
/// Structure the store each Verb's data
pub struct Verbs {
    inf: String,
    dri: String,
    pra: String,
    per: String,
    trm: String,
}
impl Verbs {
    fn new(inf: &str, dri: &str, pra: &str, per: &str, trm: &str) -> Self {
        Verbs {
            inf: inf.to_owned(),
            dri: dri.to_owned(),
            pra: pra.to_owned(),
            per: per.to_owned(),
            trm: trm.to_owned(),
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
            self.dri.bright_blue(),
            self.pra.cyan(),
            self.per.bright_magenta()
        );
    }
}

/// Getting content of Deck from file
pub fn init(path: &Path, delim: char, n: u8) -> Vec<Verbs> {
    let mut r: Vec<Verbs> = Vec::new();
    // getting contents of file
    let br = BufReader::new(File::open(path).expect("Couldn't open file, quitting..."));
    // storing lines of content
    let mut lines = br.lines();
    // ignoring properties: [crablit]
    for _ in 0..n {
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
        let trm = words.next().unwrap_or("").trim();

        let _other = words.next().unwrap_or("NNNNNN").trim();

        // making a Verbs of the values
        let tmp = Verbs::new(inf, dri, pra, per, trm);
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

/// Learning the previously initialized Deck
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
            eprintln!("Oh, no! Missing verbform found!");
            continue;
        }

        println!("\n\n{} {}", Exp::val(&Exp::Quest), trm.bright_blue());
        // printer = format!("{printer}\nsay the term for: {}\n", term.blue());
        let guess = user_input("> ");
        let guess = guess.trim();

        if guess == format!("{}, {}, {}, {}", inf, dri, pra, per) {
            println!("{} {}", Exp::val(&Exp::Knew), Exp::val(&Exp::KnewIt));
        } else if guess == "skip" {
            println!(
                "{} {:?}",
                Exp::val(&Exp::Skip),
                Verbs::new(inf, dri, pra, per, trm),
            );
            continue;
        } else if guess == "revise" {
            if r.len() == 1 {
                println!("Type revise again!");
            } else if r.is_empty() {
                println!("Nothing to revise, you might to type it again to make it work...");
            } else {
                println!("{}", Exp::val(&Exp::Revise));
            }
            break;
        } else if guess == "typo" {
            println!("{} {:?}", Exp::val(&Exp::Typo), r.pop());
            if !question(vec![Verbs::new(inf, dri, pra, per, trm)]).is_empty() {
                r.push(Verbs::new(inf, dri, pra, per, trm))
            }
        } else if guess == "hint" {
            crate::hint(inf);
            if !question(vec![Verbs::new(inf, dri, pra, per, trm)]).is_empty() {
                r.push(Verbs::new(inf, dri, pra, per, trm))
            }
        } else if guess == ":q" || guess == "quit" || guess == "exit" {
            println!("{}", Exp::val(&Exp::Exit));
            // break; <- doesn't work, as the outer while keeps repeating
            exit(0);
        } else {
            let tmp = Verbs::new(inf, dri, pra, per, trm);
            // tmp.print_all();
            print!("{} ", Exp::val(&Exp::Wrong));
            tmp.print_em();
            println!(" {}", Exp::val(&Exp::WrongIt));
            r.push(tmp);
        }
    }
    if r.len() > 1 {
        println!("\n\n{} remaining cards are {:#?}", r.len(), r);
    }

    r
}

/// Function to convert a Deck from Verbs to Cards
pub fn conv(v: &[Verbs], o: &str, delim: char) {
    let mut output = File::create(o).expect("couldn't create file!");
    // writeln!(output, "[crablit]").expect("Not succesful.");
    // writeln!(output, "[mode: cards]").expect("Not succesful.");
    // writeln!(output, "[delim: {delim}]").expect("Not succesful.");
    // writeln!(output).expect("Couldn't write to file.");

    // let has_header = true;
    // if has_header {}

    for line in v {
        writeln!(output, "{}{delim}{}", line.trm, line.inf).expect("couldn't write to file!");
    }
    println!("Converting verbs to cards written to {o}");
}
