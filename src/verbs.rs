use crate::{Colorize, Exp, File, Learn};
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
        format!("\n\n{} {}", Exp::val(&Exp::Quest), self.trm.bright_blue())
    }

    fn correct(&self) -> String {
        format!("{}, {}, {}, {}", &self.inf, &self.dri, &self.pra, &self.per)
    }

    fn skip(&self) -> String {
        format!(
            "{} {:?}",
            Exp::val(&Exp::Skip),
            Verbs::new(&self.inf, &self.dri, &self.pra, &self.per, &self.trm),
        )
    }

    fn wrong(&self) -> String {
        format!(
            "{} {} {}",
            Exp::val(&Exp::Wrong),
            self.print_em(),
            Exp::val(&Exp::WrongIt)
        )
    }

    fn hint(&self) {
        crate::hint(&self.inf);
    }

    fn new_from_line(line: &str, delim: char) -> Self {
        let mut words = line.split(delim);

        let inf = words.next().unwrap_or("").trim();
        let dri = words.next().unwrap_or("").trim();
        let pra = words.next().unwrap_or("").trim();
        let per = words.next().unwrap_or("").trim();
        let trm = words.next().unwrap_or("").trim();

        let _other = words.next().unwrap_or("NNNNNN").trim();

        // making a Verbs of the values
        Verbs::new(inf, dri, pra, per, trm)
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
pub fn conv(v: &[Verbs], o: &str, delim: char) {
    let mut output = File::create(o).expect("couldn't create file!");
    // writeln!(output, "[crablit]").expect("Not succesful.");
    // writeln!(output, "[mode: cards]").expect("Not succesful.");
    // writeln!(output, "[delim: {delim}]").expect("Not succesful.");
    // writeln!(output).expect("Couldn't write to file.");

    // let has_header = true;
    // if has_header {}

    for line in v {
        writeln!(output, "{}{delim}{}", line.trm, line.inf).expect("couldn't write to file!");
    }
    println!("Converting verbs to cards written to {}", o);
}
