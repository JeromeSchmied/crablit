use colored::Colorize;

#[derive(Debug)]
struct Cards {
    trm: String,
    def: String,
}

impl Cards {
    fn new(term: &str, def: &str) -> Self {
        Self {
            trm: term.to_string(),
            def: def.to_string(),
        }
    }
}

fn main() {
    // println!("mennyi ujja van Babineninek ropibol ugy igazan?");
    // let mut babineni = String::new();
    // std::io::stdin()
    //     .read_line(&mut babineni)
    //     .expect("what is goink on?");
    // let babineni: u32 = babineni.trim().parse().unwrap_or(0);
    // println!("babineninek {} ujja van ropibol", babineni);

    let v = vec![
        Cards {
            trm: "heloo".to_string(),
            def: "Guten Tag!".to_string(),
        },
        Cards::new("gondolat", "e Gedanke, n"),
        Cards::new("sirály", "e Möwe, n"),
        Cards::new("szobor", "e Nachdank, e"),
        Cards::new("együttes", "e Band, e"),
        Cards::new("harang", "e Glocke, n"),
    ];

    for Cards { trm, def } in &v {
        println!("say the term for: {}", trm);
        let mut guess = String::new(); // = "e Mowe, n".to_string();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("hajajajajaja");

        let guess = guess.trim();
        if def == guess {
            println!("{}", "that's about it!".green());
        } else {
            println!(
                "unfortunately no: {}: {}\nwhile your guess was: \"{}\"",
                trm,
                def.blue(),
                guess.red()
            );
        }
        println!();
    }
    // println!("\n\ncards: {:#?}", v);
    // dbg!(a);
}
