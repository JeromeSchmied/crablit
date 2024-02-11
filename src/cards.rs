//! # This module includes code specific to learning expressions.
use crate::*;
use colored::Colorize;
use std::error::Error;
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
    /// Creates new instance of a `Card`
    ///
    /// # usage
    ///
    /// ```
    /// use crablit::Card;
    ///
    /// let card = Card::new("dog", "Hunde");
    /// ```
    pub fn new(term: &str, def: &str) -> Self {
        Self {
            trm: term.to_string(),
            def: def.to_string(),
        }
    }
    /// Swaps term and definition
    ///
    /// # usage
    ///
    /// ```
    /// use crablit::cards;
    ///
    /// let mut swapd = cards::Card::new("ask", "answer");
    /// swapd.swap();
    ///
    /// assert_ne!(cards::Card::new("ask", "answer"), swapd);
    /// ```
    pub fn swap(&mut self) {
        swap(&mut self.trm, &mut self.def);
    }
}

impl Learn for Card {
    fn question(&self) -> String {
        Msg::Quest(self.trm.to_string()).val()
    }

    fn correct(&self) -> String {
        self.def.to_string()
    }

    fn skip(&self) -> String {
        Msg::Skip(self.ser(" = ")).val()
        // format!("{}{:?}", Msg::Skip.val(), self)
    }

    fn wrong(&self) -> String {
        Msg::Wrong(self.def.yellow().underline().to_string()).val()
        // format!(
        //     "{}{}{}\n",
        //     Msg::Wrong.val(),
        //     self.def.yellow().underline(),
        //     Msg::WrongIt.val()
        // )
    }

    fn hint(&self) -> String {
        Msg::Hint(crate::hint(&self.def)).val()
    }

    fn deser(line: &str, delim: char) -> Result<Box<Self>, Box<dyn Error>> {
        let mut words = line.split(delim);
        if words.clone().count() != 2 {
            Err(format!(
                "A line should look like this:\n\t\"{}{}{}\".\nInstead looks like this:\n\t\"{}\".",
                "<term>".blue().italic(),
                delim.to_string().red().bold(),
                "<definition>".yellow().italic(),
                line,
            )
            .into())
        } else {
            let trm = words.next().unwrap().trim();
            let def = words.next().unwrap().trim();
            Ok(Box::new(Card::new(trm, def)))
        }
    }

    fn flashcard(&self) -> String {
        Msg::Flash(self.def.clone()).val()
    }

    fn ser(&self, delim: &str) -> String {
        format!("{}{}{}", self.trm, delim, self.def)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let trm = "tarmak is hot";
        let def = "hot asphalt";

        assert_eq!(
            Card {
                trm: trm.to_string(),
                def: def.to_string()
            },
            Card::new(trm, def)
        )
    }

    #[test]
    fn swap() {
        let mut swapd = Card::new("ask", "answer");
        swapd.swap();

        assert_ne!(Card::new("ask", "answer"), swapd);
        assert_eq!(
            Card {
                trm: "answer".into(),
                def: "ask".into()
            },
            swapd
        );
    }

    #[test]
    fn disp() {
        let card = Card::new("term", "def");

        assert_eq!(card.question(), Msg::Quest("term".to_string()).val());
    }
}
