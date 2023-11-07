# [Learnit](https://github.com/JeromeSchmied/learnit)

## [Anki](https://ankiweb.net), [quizlet](https://quizlet.com) and [knowt](https://knowt.com) inspired app, but in the terminal

## Features
- cross-platform: compiles wherever Rust does.
- card: \<term>\<delimiter>\<definition>. eg: "to learn;lernen"
- easy deck making:
    +  file of cards with the same delimiter everywhere, good delimiters are: ` ";", "    "(tab), "-", ":"`, but could be anything.
    +  lines starting with '#' are comments
    +  some headers are accepted if the first one is learnit: mode, delimiter. They shall be in brackets. But they are not necessery!
    +  extra newlines cause no problem
    +  for instance: 
```text
[learnit]
[mode: cards]
[delimiter: -]

# example file that can be parsed by learnit

soup - Suppe
fast - schnell
alpaca - das Alpaka
proud - stolz
to pour - gießen
# I don't know what to write next. - Ich weiß nicht was...
```
- written questions: people tend to remember words better this way than flashcards
- mode for Verbs-learning: if you need to learn lots of verbforms, like:
```text
[learnit]
[mode: verbs]
[delim: 	]

# Verben tabelle von DaF leicht 1,2
# it's hungarian but that really doesn't matter

Lecke	Infinitiv	E/3	Präteritum	Perfekt	Jelentés	Egyéb
11	akzeptieren	akzeptiert	akzeptierte	hat akzeptiert	elfogad	
13	ändern	ändert	änderte	hat geändert	változtat v-t, v-n	
17	sich ändern	ändert sich	änderte sich	hat sich geändert	változik	
10	antworten	antwortet	antwortete	hat geantwortet	felel/válaszol	
14	sich ärgern	ärgert sich	ärgerte sich	hat sich geärgert	bosszankodik	(über+A)
15	atmen	atmet	atmete	hat geatmet	lélegzik	
8	baden	badet	badete	hat gebadet	fürdik
...
```
- Fast: initialization of txt file of 1.3GB size with 24379399 lines of cards took 37s with a maximum of 3.6GB ram usage. To be improved further.

## How it works
- it takes a source file: .tsv, .csv or .txt. See [examples](https://github.com/JeromeSchmied/learnit/tree/main/examples)
- stores it in a vector
- asks them until you know all well (currently only till you guess them right once)

<!-- ## Why is it better than the others? -->
<!---->
<!-- |                 | quizlet     | knowt      | learnit                    | -->
<!-- |---------------- | ----------- | ---------- | -------------------------- | -->
<!-- | open-source     | no          | no         | of course!                 | -->
<!-- | ad-free         | nope        | nope       | 100%                       | -->
<!-- | totally free    | not really  | not really | Yes, and it always will be | -->
<!-- | speed out of 10 | 4           | 2          | 10                         | -->
<!-- | offline version | paid        | no         | cross-platform, fast, TUI: coming soon  | -->

## Installing:

install Rust, if you don't have it:
- on *nix systems(linux, unix, macos): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` 
- on windows: download installer: [rustup-init.exe](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)
- when in doubt: [Install instructions](https://www.rust-lang.org/tools/install)

```bash
# clone the repo to have it locally
git clone --depth=1 https://github.com/JeromeSchmied/learnit.git
# go to it's directory, where it's been cloned
cd learnit
# compile, and run it with examples/verbs.tsv
cargo run -- examples/18_eng.txt
```

## To help, see [todos](TODO.md)

## Alternatives: 
- [speki](https://crates.io/crates/speki): only flashcards, rust
- [vocage](https://crates.io/crates/vocage): only flashcards, rust
- [fla.sh](https://github.com/tallguyjenks/fla.sh): only flashcards, bash
- [hascard](https://github.com/Yvee1/hascard): only flashcards, haskell
- [exhaust](https://github.com/heyrict/exhaust): I couldn't get it to work, rust
