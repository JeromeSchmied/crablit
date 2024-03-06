//! # This module includes code specific to learning expressions.
use crate::*;
use std::{error::Error, mem::swap};

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    /// Term in known language
    trm: String,
    /// Definition in language to be learnt
    def: String,
    /// level of knowledge: 0,1,2,3
    lok: Lok,
}
impl Card {
    /// Creates new instance of a `Card`
    ///
    /// # usage
    ///
    /// ```
    /// use crablit::Card;
    ///
    /// let card = Card::new("dog", "Hunde", None);
    /// ```
    pub fn new(term: &str, def: &str, lok: Option<&str>) -> Self {
        Self {
            trm: term.to_string(),
            def: def.to_string(),
            lok: Lok::new(lok.unwrap_or_default()),
        }
    }
    /// Swaps term and definition
    ///
    /// # usage
    ///
    /// ```
    /// use crablit::cards;
    ///
    /// let mut swapd = cards::Card::new("ask", "answer", None);
    /// swapd.swap();
    ///
    /// assert_ne!(cards::Card::new("ask", "answer", None), swapd);
    /// ```
    pub fn swap(&mut self) {
        swap(&mut self.trm, &mut self.def);
    }
}

impl Card {
    pub fn question(&self) -> String {
        format!(
            "{SPACER}{} {}",
            "?".bright_yellow().bold(),
            self.trm.bright_blue()
        )
    }

    pub fn correct(&self) -> String {
        self.def.to_string()
    }

    pub fn skip(&self) -> String {
        format!(
            "{}{} {}",
            SPACER.repeat(2),
            "Skipping:".bright_magenta(),
            self.ser(" = ")
        )
    }

    pub fn hint(&self) -> String {
        format!("{SPACER}{} {}", "#".cyan().bold(), hint(&self.def))
    }

    pub fn wrong(&self) -> String {
        format!(
            "{SPACER}{} {} {}\n\n",
            "~".bright_red().bold(),
            self.def.yellow().underline(),
            "<-is the right answer.".bright_red().italic()
        )
    }

    pub fn flashcard(&self) -> String {
        format!(
            "{SPACER}{} {}\n{SPACER}{}",
            "=".bright_cyan().bold(),
            self.def,
            "─"
                .repeat(self.def.len() + SPACER.len())
                .bright_purple()
                .bold()
        )
    }

    pub fn deser(line: &str, delim: char) -> Result<Self, Box<dyn Error>> {
        let mut words = line.split(delim);
        if words.clone().count() != 2 && words.clone().count() != 3 {
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
            let lok = words.next();
            Ok(Card::new(trm, def, lok))
        }
    }

    pub fn ser(&self, delim: &str) -> String {
        format!(
            "{}{delim}{}{delim}{}",
            self.trm,
            self.def,
            self.lok().display()
        )
    }

    pub fn incr(&mut self) {
        self.lok.incr();
    }

    pub fn decr(&mut self) {
        self.lok.decr();
    }

    pub fn lok(&self) -> Lok {
        self.lok.clone()
    }
}

pub(crate) fn deser_verbs_to_cards(
    v: &[Card],
    conf: &config::Config,
) -> Result<String, Box<dyn Error>> {
    Ok(v.iter().fold(String::new(), |result, card| {
        result
            + &format!(
                "{}{}{}\n",
                card.trm.split(conf.delim()).next().unwrap_or(""),
                conf.delim(),
                &card.def
            )
    }))
}

// pub fn deser_to_card(verbs: &[Verb], conf: &config::Config) -> Result<(), Box<dyn Error>> {
//     let pb = PathBuf::from(&conf.file_path_orig());
//     let outf_name = format!("{}_as_cards.csv", pb.file_stem().unwrap().to_str().unwrap());
//     println!(
//         "\n\nConverting verbs to cards, from file: {:?} to file: {}",
//         conf.file_path_orig(),
//         outf_name.bright_blue()
//     );
//     let mut out_f = File::create(outf_name)?;

//     writeln!(out_f, "# [crablit]")?;
//     writeln!(out_f, "# mode = \"cards\"")?;
//     writeln!(out_f, "# delim = \'{}\'\n\n", conf.delim())?;

//     for line in verbs {
//         writeln!(out_f, "{}{}{}", line.trm, conf.delim(), line.inf)?;
//     }

//     println!("Converting from verbs to cards done");

//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let trm = "tarmak is hot".to_string();
        let def = "hot asphalt".to_string();

        assert_eq!(
            Card::new(&trm, &def, None),
            Card {
                trm,
                def,
                lok: Lok::default()
            }
        );
    }

    #[test]
    fn swap() {
        let mut swapd = Card::new("ask", "answer", None);
        swapd.swap();

        assert_ne!(Card::new("ask", "answer", None), swapd);
        assert_eq!(
            Card {
                trm: "answer".into(),
                def: "ask".into(),
                lok: Lok::default()
            },
            swapd
        );
    }

    // #[test]
    // fn disp() {
    //     todo!()
    //     let card = Card::new("term", "def", None);

    //     assert_eq!(card.question(), Msg::("term".to_string()).val());
    // }
}
