//! # This module includes code specific to learning expressions.
use crate::*;
use std::mem::swap;

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    /// Term in known language
    trm: String,
    /// Definition in language to be learnt
    def: String,
    // /// level of knowledge
    // lev: u32,
}
impl Card {
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

impl Learn for Card {
    fn disp(&self) -> String {
        format!("\n{} {}", Msg::Quest.val(), self.trm.bright_blue())
    }

    fn correct(&self) -> String {
        self.def.to_string()
    }

    fn skip(&self) -> String {
        format!("{} {:?}", Msg::Skip.val(), Card::new(&self.trm, &self.def))
    }

    fn wrong(&self) -> String {
        format!(
            "{} {} {}\n",
            Msg::Wrong.val(),
            self.def.yellow().underline(),
            Msg::WrongIt.val()
        )
    }

    fn hint(&self) {
        println!("{}", crate::hint(&self.def));
    }

    fn ser(line: &str, delim: char) -> Result<Box<Self>, String> {
        let mut words = line.split(delim);
        if words.clone().count() != 2 {
            Err(format!(
                "A line should look like this:\n\t\"{}{}{}\".\nInstead looks like this:\n\t\"{}\".",
                "<term>".blue().italic(),
                delim.to_string().red().bold(),
                "<definition>".yellow().italic(),
                line,
            ))
        } else {
            let trm = words.next().unwrap().trim();
            let def = words.next().unwrap().trim();
            Ok(Box::new(Card::new(trm, def)))
        }
    }

    // fn deserialize<T: Learn>(&self, v: &[T]) -> Result<String, Box<dyn Error>> {
    //     for card in v {
    //         // format!("{}{}{}", card)
    //     }
    //     todo!()
    // }

    fn flashcard(&self) -> String {
        let s = &self.def;
        let r = "─".repeat(s.len() + 4);
        format!("{}\n{}", s, r.bright_purple().bold())
    }

    fn deser(&self, delim: char) -> String {
        format!("{}{}{}", self.trm, delim, self.def)
    }
}
