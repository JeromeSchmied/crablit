use crate::{Colorize, Exp, Learn};
use std::mem::swap;

#[derive(Debug, Clone)]
pub struct Cards {
    /// term in known language
    trm: String,
    /// definition in language to be learnt
    def: String,
    // /// level of knowledge
    // lev: u32,
}
impl Cards {
    pub fn new(term: &str, def: &str) -> Self {
        Self {
            trm: term.to_string(),
            def: def.to_string(),
        }
    }
    pub fn swap(&mut self) {
        swap(&mut self.trm, &mut self.def);
    }
    // pub fn trm(self) -> String {
    //     self.trm
    // }
    // pub fn def(self) -> String {
    //     self.def
    // }
}

impl Learn for Cards {
    fn show(&self) -> String {
        format!("\n{} {}", Exp::val(&Exp::Quest), self.trm.bright_blue())
    }

    fn correct(&self) -> String {
        self.def.to_string()
    }

    fn skip(&self) -> String {
        format!(
            "{} {:?}",
            Exp::val(&Exp::Skip),
            Cards::new(&self.trm, &self.def)
        )
    }

    fn wrong(&self) -> String {
        format!(
            "{} {} {}\n",
            Exp::val(&Exp::Wrong),
            self.def.yellow(),
            Exp::val(&Exp::WrongIt)
        )
    }

    fn hint(&self) {
        crate::hint(&self.def);
    }

    fn new_from_line(line: &str, delim: char) -> Self {
        let mut words = line.split(delim);
        let trm = words.next().unwrap_or("NO_TERM").trim();
        // ignoring newlines, lines starting with #
        let def = words.next().unwrap_or("NO_DEFINITION").trim();
        Cards::new(trm, def)
    }

    // fn copy(&self) -> Self {
    //     Cards::new(&self.trm, &self.def)
    // }
}

// /// Getting content of Deck from file.
// pub fn init(path: &Path, delim: char, n: u8) -> Vec<Cards> {
//     let mut tmp_vec: Vec<Cards> = Vec::new();
//     let br = BufReader::new(File::open(path).expect("Couldn't open file, quitting..."));
//     // let contents = fs::read_to_string(path).expect("what is wronk?");
//     // storing lines of contents
//     // let mut limes = contents.lines();
//     let mut lines = br.lines();
//     for _ in 0..n {
//         lines.next();
//     }
//     // iterating over the lines of file to store them in a vector
//     for line in lines {
//         let line = line.expect("Something wrong with bufread line");
//         let mut words = line.split(delim);

//         let trm = words.next().unwrap_or("NO_TERM").trim();
//         // ignoring newlines, lines starting with #
//         if trm.is_empty() || trm.starts_with('#') {
//             continue;
//         };

//         let def = words.next().unwrap_or("NO_DEFINITION").trim();

//         let tmp = Cards::new(trm, def);
//         tmp_vec.push(tmp);
//     }
//     println!("{:?} file succesfully read.", path);
//     // println!("Basic file looks somehow like this:\n{}", contents);
//     // for card in &tmp_vec {
//     //     println!("\"{}\":\t\t\t\"{}\"", card.trm.yellow(), card.def.magenta());
//     // }
//     tmp_vec
// }

// /// Learning the previously initialized Deck
// pub fn question(v: Vec<Cards>) -> Vec<Cards> {
//     // let mut printer = String::new();
//     if v.len() != 1 {
//         println!("\n\nYou have {} words to learn, let's start!", v.len());
//     }
//     let mut r: Vec<Cards> = Vec::new();

//     for Cards {
//         trm: term,
//         def: defi,
//     } in &v
//     {
//         if defi.is_empty() || defi == "NO_DEFINITION" || term.is_empty() || term == "NO_TERM" {
//             println!("{}", "Oh, no! Missing word found!".bright_red());
//             dbg!(&defi);
//             dbg!(&term);
//             continue;
//         }
//         println!("\n{} {}", Exp::val(&Exp::Quest), term.bright_blue());
//         let guess = user_input("> ");
//         let guess = guess.trim();

//         // printer = format!("{printer}{}", guess);
//         if guess == defi {
//             // printer = format!("{printer}{}\n", "that's about it!".bright_green());
//             println!("{} {}\n", Exp::val(&Exp::Knew), Exp::val(&Exp::KnewIt));
//         } else if guess == "skip" {
//             println!("{} {:?}", Exp::val(&Exp::Skip), Cards::new(term, defi));
//             continue;
//         // } else if guess == "repeat" || guess == "rep" {
//         //     let prev = r.last().unwrap_or(&Cards::new(term, defi));
//         //     if !question(vec![r.last().unwrap()]).is_empty() {
//         //         r.push(r.last().expect("Should have at least one element"));
//         //     }
//         } else if guess == "revise" {
//             if r.len() == 1 {
//                 println!("Type revise again!");
//             } else if r.is_empty() {
//                 println!("Nothing to revise, you might to type it again to make it work...");
//             } else {
//                 println!("{}", Exp::val(&Exp::Revise));
//             }
//             break;
//         } else if guess == "typo" {
//             // printer = format!("{printer}\n{}{:?}", "Corrected!".magenta(), r.last());
//             println!("{}{:?}", Exp::val(&Exp::Typo), r.pop());
//             if !question(vec![Cards::new(term, defi)]).is_empty() {
//                 r.push(Cards::new(term, defi));
//             }
//         } else if guess == ":q" || guess == "quit" || guess == "exit" {
//             println!("{}", Exp::val(&Exp::Exit));
//             exit(0);
//         } else if guess == "hint" {
//             crate::hint(defi);

//             if !question(vec![Cards::new(term, defi)]).is_empty() {
//                 r.push(Cards::new(term, defi));
//             }
//         // treat them as flashcarding
//         // } else if guess == "" {
//         //     println!(
//         //         "{} {}\n{}\n\n\n",
//         //         Qot::val(&Qot::Flash),
//         //         defi.cyan().bold(),
//         //         "───────────────────".bright_purple()
//         //     );
//         } else {
//             r.push(Cards::new(term, defi));
//             // printer = format!(
//             //     "{printer}unfortunately no: {}: {}\nwhile your guess was: \"{}\"\n",
//             //     term.yellow(),
//             //     defi.blue(),
//             //     guess.red()
//             // );
//             println!(
//                 "{} {} {}\n",
//                 Exp::val(&Exp::Wrong),
//                 defi.yellow(),
//                 Exp::val(&Exp::WrongIt)
//             );
//         }
//         // println!("{:#>width$}\n\n", "#".magenta(), width = guess.len() + 12);
//         // clearscreen::clear().expect("failed to clear screen");
//         // println!("{printer}\n\n\n\n");
//         // if printer.len() > 2 * 90 {
//         //     for _ in 0..6 {
//         //         let pos = printer.trim_start().find('\n').unwrap_or(80);
//         //         printer.drain(..pos);
//         //     }
//         // }
//     }
//     if r.len() > 1 {
//         println!("\n\n{} remaining cards are {:#?}", r.len(), r);
//     }
//     r
// }
