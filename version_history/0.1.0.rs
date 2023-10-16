use colored::Colorize;
use std::fs::File;
use std::io::read_to_string;
use std::path::Path;

#[derive(Debug)]
struct Cards {
    trm: String,
    def: String,
}

impl Cards {
    fn new(term: &str, def: &str) -> Self {
        Self {
            trm: term.to_string(),
            def: def.to_string(),
        }
    }
}

enum Kards {
    Adjektiv(String),
    Nomen(String),
    Verb {
        inf: String,
        dri: String,
        pra: String,
        per: String,
    },
}

fn main() {
    // hardcoded();
    let fname = user_input("What's the filename containing flashcards to be learnt?");
    let delim = user_input("What character is the delimiter?");
    // let v = init(&fname, delim.chars().nth(0).unwrap_or(';'));
    let p = Path::new(&fname);
    let v = init(p, delim.chars().nth(0).unwrap_or(';'));
    question(v);
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

fn user_input_int() {
    println!("mennyi ujja van Babineninek ropibol ugy igazan?");
    let mut babineni = String::new();
    std::io::stdin()
        .read_line(&mut babineni)
        .expect("what is goink on?");
    let babineni: u32 = babineni.trim().parse().unwrap_or(0);
    println!("babineninek {} ujja van ropibol", babineni);
}

fn hardcoded() {
    let v = vec![
        Cards {
            trm: "heloo".to_string(),
            def: "Guten Tag!".to_string(),
        },
        Cards::new("gondolat", "e Gedanke, n"),
        Cards::new("sirály", "e Möwe, n"),
        Cards::new("szobor", "e Nachdank, e"),
        Cards::new("együttes", "e Band, e"),
        Cards::new("harang", "e Glocke, n"),
    ];
    question(v);
}

fn init(fname: &Path, delim: char) -> Vec<Cards> {
    let mut tmp_vec: Vec<Cards> = Vec::new();
    println!("Trying to open {:?}", fname);
    let file = File::open(fname).expect("Nincs fajl!");
    for line in read_to_string(file).expect("bajos fajl!\n").lines() {
        let mut words = line.split(delim);
        let trm = words.next().unwrap_or("NANANA");
        let def = words.next().unwrap_or("NANANA");
        let tmp = Cards {
            trm: trm.to_string(),
            def: def.to_string(),
        };
        tmp_vec.push(tmp);
    }
    println!("{:?} file succesfully readed.", fname);
    // println!("It looks somehow like this: {:#?}", tmp_vec);
    for Cards { trm, def } in &tmp_vec {
        // let num = def.len() + 20 - trm.len();
        // println!("{}:{def:>num$}", trm.yellow());
        println!("{}:\t{def}", trm.yellow());
    }
    tmp_vec
}

fn question(v: Vec<Cards>) {
    for Cards { trm, def } in &v {
        println!("say the term for: {}", trm);
        let mut guess = String::new(); // = "e Mowe, n".to_string();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("hajajajajaja");

        let guess = guess.trim();
        // println!("your guess: {}", guess.blue());
        if def == guess {
            println!("{}", "that's about it!".green());
        } else if guess == "exit" {
            println!("exiting...");
            break;
        } else {
            println!(
                "unfortunately no: {}: {}\nwhile your guess was: \"{}\"",
                trm.yellow(),
                def.blue(),
                guess.red()
            );
        }
        println!();
    }
    // println!("\n\nAll cards are: {:#?}", v);
}
