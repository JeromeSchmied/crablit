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
            self.def.yellow().underline(),
            Exp::val(&Exp::WrongIt)
        )
    }

    fn hint(&self) {
        crate::hint(&self.def);
    }

    fn new_from_line(line: &str, delim: char) -> Self {
        let mut words = line.split(delim);
        if words.clone().count() != 2 {
            panic!(
                "{:?} line should consist of a {}{}{}.",
                &words,
                "<term>".blue().italic(),
                "<delimiter>".red().bold(),
                "<definition>".yellow().italic()
            );
        } else {
            let trm = words.next().unwrap().trim();
            let def = words.next().unwrap().trim();
            Cards::new(trm, def)
        }
    }

    fn flashcard(&self) -> String {
        let s = &self.def;
        let mut r = String::new();
        for _ in 0..s.chars().count() + 4 {
            r.push('─');
        }
        format!("{}\n{}", s, r.bright_purple().bold())
    }

    // fn copy(&self) -> Self {
    //     Cards::new(&self.trm, &self.def)
    // }
}
