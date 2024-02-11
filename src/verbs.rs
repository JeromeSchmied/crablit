//! # This module includes code specific to learning verbforms.
use crate::{config::Config, *};
use colored::Colorize;
use std::io::Write;

#[derive(Debug, Clone)]
/// Structure the store each Verb's data
pub struct Verb {
    inf: String,
    dri: String,
    pra: String,
    per: String,
    trm: String,
}
impl Verb {
    pub fn new(inf: &str, dri: &str, pra: &str, per: &str, trm: &str) -> Self {
        Verb {
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
    fn print_em(&self) -> String {
        format!(
            "{}, {}, {}, {}",
            self.inf.yellow().underline(),
            self.dri.bright_blue().underline(),
            self.pra.cyan().underline(),
            self.per.bright_magenta().underline()
        )
    }
}

impl Learn for Verb {
    fn question(&self) -> String {
        format!("\n\n{} {}", Msg::Quest.val(), self.trm.bright_blue())
    }

    fn correct(&self) -> String {
        format!("{}, {}, {}, {}", &self.inf, &self.dri, &self.pra, &self.per)
    }

    fn skip(&self) -> String {
        format!(
            "{} {:?}",
            Msg::Skip.val(),
            Verb::new(&self.inf, &self.dri, &self.pra, &self.per, &self.trm),
        )
    }

    fn wrong(&self) -> String {
        format!(
            "{} {} {}",
            Msg::Wrong.val(),
            self.print_em(),
            Msg::WrongIt.val()
        )
    }

    fn hint(&self) {
        println!("{} {}", Msg::Hint.val(), crate::hint(&self.inf));
    }

    fn deser(line: &str, delim: char) -> Result<Box<Self>, Box<dyn Error>> {
        let mut words = line.split(delim);

        let inf = words.next().unwrap_or("").trim();
        let dri = words.next().unwrap_or("").trim();
        let pra = words.next().unwrap_or("").trim();
        let per = words.next().unwrap_or("").trim();
        let trm = words.next().unwrap_or("").trim();

        if inf.is_empty() || dri.is_empty() || pra.is_empty() || per.is_empty() || trm.is_empty() {
            Err(format!(
                "A line should look like this: \n\t\"{}{}{}{}{}{}{}{}{}\".\nInstead looks like this: \n\t\"{}\".",
                "<infinitive>".yellow().italic(),
                delim.to_string().red().bold(),
                "<3rd person>".red().bold(),
                delim.to_string().red().bold(),
                "<simple past>".green().bold(),
                delim.to_string().red().bold(),
                "<present perfect>".red().bold(),
                delim.to_string().red().bold(),
                "<term>".blue().italic(),
                line,
            ).into())
        } else {
            // making a Verbs of the values
            Ok(Box::new(Verb::new(inf, dri, pra, per, trm)))
        }
    }

    fn flashcard(&self) -> String {
        let s = format!("{}, {}, {}, {}", &self.inf, &self.dri, &self.pra, &self.per);
        let r = "─".repeat(s.len() + 4);
        format!("{}\n{}", s, r.bright_purple().bold())
    }

    fn ser(&self, delim: char) -> String {
        format!(
            "{}{delim}{}{delim}{}{delim}{}{delim}{}",
            self.inf, self.dri, self.pra, self.per, self.trm
        )
    }
}

/// Function to convert a Deck from Verbs to Cards
pub fn deser_to_card(verbs: &[Verb], conf: &Config) -> Result<(), Box<dyn Error>> {
    let pb = PathBuf::from(&conf.file_path_orig());
    let outf_name = format!("{}_as_cards.csv", pb.file_stem().unwrap().to_str().unwrap());
    println!(
        "\n\nConverting verbs to cards, from file: {:?} to file: {}",
        conf.file_path_orig(),
        outf_name.bright_blue()
    );
    let mut out_f = File::create(outf_name)?;

    writeln!(out_f, "# [crablit]")?;
    writeln!(out_f, "# mode = \"cards\"")?;
    writeln!(out_f, "# delim = \'{}\'\n\n", conf.delim())?;

    for line in verbs {
        writeln!(out_f, "{}{}{}", line.trm, conf.delim(), line.inf)?;
    }

    println!("Converting from verbs to cards done");

    Ok(())
}
