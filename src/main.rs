use rand::{seq::SliceRandom, thread_rng};
use std::env;
use std::path::Path;

pub mod basic;
pub mod flashcards;
pub mod verbs;

// enum Kards {
//     Adjektiv(String),
//     Nomen(String),
//     Verb {
//         inf: String,
//         dri: String,
//         pra: String,
//         per: String,
//     },
//     Wendungen(String),
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let mode: String = match args.get(1) {
        Some(m) => m.to_owned(),
        None => {
            let m = flashcards::user_input("Mode(verb,basic): ");
            m
        }
    };
    let file_path = match args.get(2) {
        Some(f) => f.to_owned(),
        None => {
            let fp = flashcards::user_input("File path: ");
            fp
        }
    };

    let p = Path::new(&file_path);
    if mode == "verb" || mode == "verben" || mode == "verbs" {
        let mut v = verbs::init(p);
        while !v.is_empty() {
            v.shuffle(&mut thread_rng());
            v = verbs::question(v);
        }
    } else if mode == "basic" || mode == "flashcards" || mode == "cards" || mode.is_empty() {
        let mut v = basic::init(p);
        while !v.is_empty() {
            v.shuffle(&mut thread_rng());
            v = basic::question(v);
        }
    } else {
        println!("unfortunately no mode found.\nquitting...");
    }
}
