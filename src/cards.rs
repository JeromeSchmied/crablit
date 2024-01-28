//! # This module includes code specific to learning expressions.
use crate::*;
use std::mem::swap;

#[derive(Debug, Clone, PartialEq)]
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
}

impl Learn for Cards {
    fn show(&self) -> String {
        format!("\n{} {}", Exp::Quest.val(), self.trm.bright_blue())
    }

    fn correct(&self) -> String {
        self.def.to_string()
    }

    fn skip(&self) -> String {
        format!("{} {:?}", Exp::Skip.val(), Cards::new(&self.trm, &self.def))
    }

    fn wrong(&self) -> String {
        format!(
            "{} {} {}\n",
            Exp::Wrong.val(),
            self.def.yellow().underline(),
            Exp::WrongIt.val()
        )
    }

    fn hint(&self) {
        println!("{}", crate::hint(&self.def));
    }

    fn new_from_line(line: &str, delim: char) -> Result<Box<Self>, String> {
        let mut words = line.split(delim);
        if words.clone().count() != 2 {
            Err(format!(
                "{:?} line should consist of a {}{}{}.\nInstead looks like this: {}",
                &words,
                "<term>".blue().italic(),
                "<delimiter>".red().bold(),
                "<definition>".yellow().italic(),
                line,
            ))
        } else {
            let trm = words.next().unwrap().trim();
            let def = words.next().unwrap().trim();
            Ok(Box::new(Cards::new(trm, def)))
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
}
