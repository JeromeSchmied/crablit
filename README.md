# [Crablit](https://github.com/JeromeSchmied/crablit)

## [Anki](https://ankiweb.net), [quizlet](https://quizlet.com) and [knowt](https://knowt.com) inspired learning app, but in the terminal.

## Still in beta, functionality is not tested thoroughly.

## Features

- *Cross-platform*: compiles where Rust does.
- Written answers: people tend to remember words *better* this way than flashcards.
- *Fast*: initialization of **20000000** cards (574MB) takes about **5s** with a maximum of 2.12GB ram usage.
- *Easy deck making*, in your favourite editor:
    + Simple `.txt`, `.csv` or `.tsv` file.
    + Card: `<term><delimiter><definition>`. eg: `"to learn;lernen"`.
    + File of Cards with the same delimiter in every line, good ones are: ` ";", "    "(tab), "|", ":"`.
    + Lines starting with `#` are considered comments.
    + Extra newlines cause no problem.
    + Extra spaces around delimiter cause no problem, such as: `hooray | booyah!`.
    + For instance:
```text
# example file that can be parsed by crablit

soup | Suppe

fast | schnell
alpaca | das Alpaka
proud | stolz
to pour | gießen
# I don't know what to write next. | Ich weiß nicht was...
```
- Mode for *Verb-learning*: if you need to learn lots of verbforms, like:
```text
[crablit]
[mode: verbs]
[delim: 	]

# Verben Tabelle von DaF leicht 1,2
# it's hungarian but that really doesn't matter

Infini	Dritte	Präter	Perfekt 	Jelentés	Komment
atmen	atmet	atmete	h. geatmet	lélegzik	Er hat ruhig geatmet.
baden	badet	badete	h. gebadet	fürdik	
bauen	baut	baute	h. gebaut	épít	Haus bauen
...
```
<!--+  If the first line is [crablit]: mode, delimiter may be set-->
<!-- ## Why is it better than the others? -->
<!---->
<!-- |                 | quizlet     | knowt      | crablit                                 | -->
<!-- |---------------- | ----------- | ---------- | --------------------------              | -->
<!-- | open-source     | no          | no         | of course!                              | -->
<!-- | ad-free         | nope        | nope       | 100%                                    | -->
<!-- | totally free    | not really  | not really | Yes, and it always will be              | -->
<!-- | speed out of 10 | 4           | 2          | 10                                      | -->
<!-- | offline version | paid        | no         | cross-platform, fast, TUI: coming soon  | -->

## Installing:

- Install Rust, if you don't have it: go to the official [install instructions](https://www.rust-lang.org/tools/install)
- Open a *terminal* and do one of the following install methods:
1. From [crates.io](https://crates.io/crates/crablit): ***easiest, recommended!***
```shell
# get binary
cargo install crablit
# run it with desired file containing deck of cards
crablit my_vocab_file.tsv
```
2. Install source from github:
```shell
# if on windows: curl.exe
curl -L "https://github.com/JeromeSchmied/crablit/archive/main.tar.gz" | tar -xzf -
cd crablit-main
# running goes like this, with example file:
cargo run -- examples/18_eng_deu.txt
```
3. Or you may install it with [git](https://git-scm.com/downloads), and clone the source with it:
```shell
# once you have git, clone the repo to have it locally
git clone --depth=1 https://github.com/JeromeSchmied/crablit.git
# go to the directory where it's been cloned
cd crablit
# running goes like this, with example file:
cargo run -- examples/18_eng_deu.txt
```

## Usage

- Run `crablit --help` to see options.
- If `NO_COLOR=1` coloring is disabled, thanks to [colored](https://crates.io/crates/colored).
- Type the definition of the questioned word/phrase.
- To see hint: `hint`.
- If you mistyped it, type: `typo`.
- To skip: `skip`.
- To quit: `quit` or `exit` or `:q`.

## How it works

- It takes a source text file with deck of cards: .tsv, .csv or .txt. See [examples](https://github.com/JeromeSchmied/crablit/tree/main/examples).
- Stores them in a vector.
- Asks them until you know all well (currently only till you guess them right once).

## Notes

- I'm only learning Rust at the moment, so code quality might not be outstanding.
- TUI, flashcards are coming, but it takes time.
- Any bugs, questions shall be reported to [github](https://github.com/JeromeSchmied/crablit), or [email](mailto:iITsnot.me214@proton.me).
- To help with development, see [todos.](TODO.md)

## Alternatives: 

- [speki](https://crates.io/crates/speki): only flashcards, huge download size, tui, rust
- [vocage](https://crates.io/crates/vocage): only flashcards, tui, rust
- [flcard](https://crates.io/crates/flcard): only flashcards, very simple, rust
- [fla.sh](https://github.com/tallguyjenks/fla.sh): only flashcards, bash
- [hascard](https://github.com/Yvee1/hascard): only flashcards, haskell
- [exhaust](https://github.com/heyrict/exhaust): I couldn't get it to work, rust
