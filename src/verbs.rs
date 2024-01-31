//! # This module includes code specific to learning verbforms.
use crate::*;
use std::io::Write;

#[derive(Debug, Clone)]
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

impl Learn for Verbs {
    fn show(&self) -> String {
        format!("\n\n{} {}", Exp::Quest.val(), self.trm.bright_blue())
    }

    fn correct(&self) -> String {
        format!("{}, {}, {}, {}", &self.inf, &self.dri, &self.pra, &self.per)
    }

    fn skip(&self) -> String {
        format!(
            "{} {:?}",
            Exp::Skip.val(),
            Verbs::new(&self.inf, &self.dri, &self.pra, &self.per, &self.trm),
        )
    }

    fn wrong(&self) -> String {
        format!(
            "{} {} {}",
            Exp::Wrong.val(),
            self.print_em(),
            Exp::WrongIt.val()
        )
    }

    fn hint(&self) {
        println!("{}", crate::hint(&self.inf));
    }

    fn new_from_line(line: &str, delim: char) -> Result<Box<Self>, String> {
        let mut words = line.split(delim);

        let inf = words.next().unwrap_or("").trim();
        let dri = words.next().unwrap_or("").trim();
        let pra = words.next().unwrap_or("").trim();
        let per = words.next().unwrap_or("").trim();
        let trm = words.next().unwrap_or("").trim();

        let _other = words.next().unwrap_or("NNNNNN").trim();

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
            ))
        } else {
            // making a Verbs of the values
            Ok(Box::new(Verbs::new(inf, dri, pra, per, trm)))
        }
    }

    fn flashcard(&self) -> String {
        let s = format!("{}, {}, {}, {}", &self.inf, &self.dri, &self.pra, &self.per);
        let mut r = String::new();
        for _ in 0..s.len() + 4 {
            r.push('─');
        }
        format!("{}\n{}", s, r.bright_purple().bold())
    }
}

/// Function to convert a Deck from Verbs to Cards
pub fn conv(verbs: &[Verbs], out_file_path: &str, delim: char) -> Result<(), Box<dyn Error>> {
    let mut outfile = File::create(out_file_path)?;

    writeln!(outfile, "# [crablit]")?;
    writeln!(outfile, "# mode = \"cards\"")?;
    writeln!(outfile, "# delim = \'{delim}\'\n\n")?;

    for line in verbs {
        writeln!(outfile, "{}{delim}{}", line.trm, line.inf)?;
    }

    println!("Converting from verbs to cards done");

    Ok(())
}
