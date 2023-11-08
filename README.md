# [Crablit](https://github.com/JeromeSchmied/crablit)

## [Anki](https://ankiweb.net), [quizlet](https://quizlet.com) and [knowt](https://knowt.com) inspired learning app, but in the terminal

## Features
- cross-platform: compiles wherever Rust does.
- card: \<term>\<delimiter>\<definition>. eg: "to learn;lernen"
- written questions: people tend to remember words better this way than flashcards
- Fast: initialization of txt file of 1.3GB size with 24379399 lines of cards took 37s with a maximum of 3.6GB ram usage.
- easy deck making:
    +  file of cards with the same delimiter everywhere, good delimiters are: ` ";", "    "(tab), "-", ":"`, but could be anything.
    +  lines starting with `#` are comments
    +  some headers are accepted if the first one is crablit: mode, delimiter. They shall be in brackets. But they are not necessery!
    +  extra newlines cause no problem
    +  for instance: 
```text
[crablit]
[mode: cards]
[delimiter: -]

# example file that can be parsed by crablit

soup - Suppe
fast - schnell
alpaca - das Alpaka
proud - stolz
to pour - gießen
# I don't know what to write next. - Ich weiß nicht was...
```
- mode for Verbs-learning: if you need to learn lots of verbforms, like:
```text
[crablit]
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

## How it works
- it takes a source file: .tsv, .csv or .txt. See [examples](https://github.com/JeromeSchmied/crablit/tree/main/examples)
- stores it in a vector
- asks them until you know all well (currently only till you guess them right once)

<!-- ## Why is it better than the others? -->
<!---->
<!-- |                 | quizlet     | knowt      | crablit                    | -->
<!-- |---------------- | ----------- | ---------- | -------------------------- | -->
<!-- | open-source     | no          | no         | of course!                 | -->
<!-- | ad-free         | nope        | nope       | 100%                       | -->
<!-- | totally free    | not really  | not really | Yes, and it always will be | -->
<!-- | speed out of 10 | 4           | 2          | 10                         | -->
<!-- | offline version | paid        | no         | cross-platform, fast, TUI: coming soon  | -->

## Installing:

- install Rust, if you don't have it:
go to the official [install instructions](https://www.rust-lang.org/tools/install)
- run the following to get the source:
```shell
curl -L  "https://github.com/JeromeSchmied/crablit/archive/main.tar.gz" | tar -xzf -
cd crablit-main
```
- or you may install git: [downloads](https://git-scm.com/downloads)
```shell
# if you have git, clone the repo to have it locally
git clone --depth=1 https://github.com/JeromeSchmied/crablit.git
# go to it's directory, where it's been cloned
cd crablit
```
- compile, and run it with examples/verbs.tsv
```shell
cargo run -- examples/18_eng.txt
```


## To help, see [todos](TODO.md)

## Alternatives: 
- [speki](https://crates.io/crates/speki): only flashcards, rust
- [vocage](https://crates.io/crates/vocage): only flashcards, rust
- [fla.sh](https://github.com/tallguyjenks/fla.sh): only flashcards, bash
- [hascard](https://github.com/Yvee1/hascard): only flashcards, haskell
- [exhaust](https://github.com/heyrict/exhaust): I couldn't get it to work, rust
