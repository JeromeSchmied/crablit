//! # This module includes code specific to learning expressions.
use crate::*;

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
    /// use crablit::Card;
    ///
    /// let mut swapd = Card::new("ask", "answer", None);
    /// swapd.swap_me();
    ///
    /// assert_ne!(Card::new("ask", "answer", None), swapd);
    /// ```
    pub fn swap_me(&mut self) {
        std::mem::swap(&mut self.trm, &mut self.def);
    }
}

impl Card {
    /// Returns the question for this [`Card`].
    pub fn question(&self) -> String {
        format!(
            "{SPACER}{} {}",
            "?".bright_yellow().bold(),
            self.trm.bright_blue()
        )
    }

    /// Returns the correct answer for this [`Card`].
    pub fn correct(&self) -> String {
        self.def.to_string()
    }

    /// Returns the text for skipping this [`Card`].
    pub fn skip(&self) -> String {
        format!(
            "{}{} {}",
            SPACER.repeat(2),
            "Skipping:".bright_magenta(),
            self.ser(" = ")
        )
    }

    /// Returns hint for this [`Card`].
    pub fn hint(&self) -> String {
        let hint = {
            let n = self.def.chars().count() / 2;
            [
                self.def.chars().take(n).collect::<String>(),
                self.def.chars().skip(n).map(|_| '_').collect(),
            ]
            .concat()
        };
        format!("{SPACER}{} {}", "#".cyan().bold(), hint)
    }

    /// Returns the text when this [`Card`] was wrong.
    pub fn wrong(&self) -> String {
        format!(
            "{SPACER}{} {} {}\n\n",
            "~".bright_red().bold(),
            self.def.yellow().underline(),
            "<-is the right answer.".bright_red().italic()
        )
    }

    /// Returns the flashcard text for this [`Card`].
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

    /// Deserialize this [`Card`].
    ///
    /// # Errors, Panics
    ///
    /// Errors, Panics if invalid.
    pub fn deser(line: &str, delim: char) -> AnyErr<Self> {
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

    /// Serialize this [`Card`].
    pub fn ser(&self, delim: &str) -> String {
        format!(
            "{}{delim}{}{delim}{}",
            self.trm,
            self.def,
            self.lok().display()
        )
    }

    /// Increment the [`Lok`] of this [`Card`].
    pub fn incr(&mut self) {
        self.lok.incr();
    }

    /// Decrement the [`Lok`] of this [`Card`].
    pub fn decr(&mut self) {
        self.lok.decr();
    }

    /// Returns the [`Lok`] of this [`Card`].
    pub fn lok(&self) -> Lok {
        self.lok.clone()
    }
}

/// Deserialize verbs to cards.
pub(crate) fn deser_verbs_to_cards(v: &[Card], conf: &config::Config) -> AnyErr<String> {
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

/// Swap definition and term of deck(vector) of cards
///
/// # usage
/// ```
/// use crablit::Card;
///
/// let mut deck = vec![Card::new("term1", "def1", None), Card::new("term2", "def2", None), Card::new("term3", "def3", None)];
///
/// crablit::cards::swap(&mut deck);
/// ```
pub fn swap(cards: &mut [cards::Card]) {
    cards.iter_mut().for_each(cards::Card::swap_me);
}

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
    fn swap_works() {
        let mut swapd = Card::new("ask", "answer", None);
        swapd.swap_me();

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
    #[test]
    fn swap_cards_works() {
        let mut cards = vec![Card::new("term", "definition", None)];

        swap(&mut cards);
        assert_eq!(cards, vec![Card::new("definition", "term", None)]);
    }

    // #[test]
    // fn disp() {
    //     todo!()
    //     let card = Card::new("term", "def", None);

    //     assert_eq!(card.question(), Msg::("term".to_string()).val());
    // }
}
