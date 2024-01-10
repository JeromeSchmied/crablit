# [Crablit](https://github.com/JeromeSchmied/crablit): Learning app inspired by [anki](https://ankiweb.net), [quizlet](https://quizlet.com) and [knowt](https://knowt.com), but in the terminal.

> ## IMPORTANT!
>
> Crablit is still in beta, functionality is not tested thoroughly.

## Features

-   open-source, MIT licensed
-   _Cross-platform_: compiles where Rust does, but GNU/Linux is the no. 1 priority.
-   _Fast_: initialization of **20000000** cards (574MB) takes about **5s** with a maximum of 2.12GB ram usage.
-   Written answers: people tend to remember words _better_ this way than flashcards.

## Creating files to learn

-   _Easy deck making_, in your favourite editor:
    -   Simple `.txt`, `.csv` or `.tsv` file.
    -   Card: `<term><delimiter><definition>`. eg: `"to learn;lernen"`.
    -   File of Cards with the same delimiter in every line.
    -   Good delimiters are: ` ";", "|", "    "(tab), "="`.
    -   Lines starting with `#` are considered to be comments.
    -   Extra newlines cause no problem.
    -   Extra spaces around delimiter cause no problem, such as: `hooray | booyah!`.
    -   For instance:

```text
# example file that can be parsed by crablit

soup | Suppe

fast | schnell
alpaca | das Alpaka
proud | stolz
# I don't know what to write next. | Ich weiß nicht was...
to pour | gießen
```

-   Mode for _Verb-learning_: if you need to learn lots of verbforms, like:

```text
[crablit]
[mode: verbs]
[delim: 	]

# Verben Tabelle von DaF leicht 1,2
# it's hungarian but that really doesn't matter

INFINI	DRITTE	PRÄTER	PERFEKT 	MEANING 	KOMMENT
atmen	atmet	atmete	h. geatmet	lélegzik	Er hat ruhig geatmet.
baden	badet	badete	h. gebadet	fürdik  	Das Mädchen duscht den Hund.
bauen	baut	baute	h. gebaut	épít    	Haus bauen
...
```

<!-- ## Why is it better than the others? -->
<!-- |                 | quizlet     | knowt      | crablit                       | anki          | -->
<!-- |---------------- | ----------- | ---------- | --------------------------              | -->
<!-- | open-source     | no          | no         | of course!                              | -->
<!-- | ad-free         | nope        | nope       | 100%                                    | -->
<!-- | totally free    | not really  | not really | Yes, and it always will be              | -->
<!-- | speed out of 10 | 4           | 2          | 10                                      | -->
<!-- | offline version | paid        | no         | cross-platform, fast, TUI,GUI : coming soon  | -->
<!-- # or you can install latest development version with -->
<!-- cargo install --git https://github.com/jeromeschmied/crablit -->

## Installing:

-   Install Rust, if you don't have it: go to the official [install instructions](https://www.rust-lang.org/tools/install)
-   Open a _terminal_ and do one of the following install methods:

1. From [crates.io](https://crates.io/crates/crablit): **_easiest, recommended!_**

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

-   Run `crablit --help` to see options.
-   If `NO_COLOR=1`, coloring is disabled, thanks to [colored](https://crates.io/crates/colored).
-   Type the definition of the questioned word/phrase.
-   To see hint: `:hint` or `:h`.
-   If you mistyped it, type: `:typo`.
-   To skip: `:skip`.
-   To quit: `quit` or `exit` or `:q`.

## How it works

![Sample][1]

-   See [images for more](img)!
-   It takes a source text file with deck of cards: .tsv, .csv or .txt. See [examples](https://github.com/JeromeSchmied/crablit/tree/main/examples).
-   Stores them in a vector.
-   Asks them until you know all well (currently only till you guess them right once).

## Notes

-   I'm only learning Rust at the moment, so code quality might not be outstanding.
-   flashcards, TUI, GUI, Website are coming, but it takes time.
-   Any bugs, questions shall be reported to [github](https://github.com/JeromeSchmied/crablit), or [email](mailto:iITsnot.me214@proton.me).
-   To help with development, see [todos](TODO.md).

## Alternatives:

-   [speki](https://crates.io/crates/speki): only flashcards, huge download size, tui, rust
-   [vocage](https://crates.io/crates/vocage): only flashcards, tui, rust
-   [flcard](https://crates.io/crates/flcard): only flashcards, very simple, rust
-   [fla.sh](https://github.com/tallguyjenks/fla.sh): only flashcards, bash
-   [hascard](https://github.com/Yvee1/hascard): only flashcards, haskell
-   [exhaust](https://github.com/heyrict/exhaust): I couldn't get it to work, rust

[1]: img/v0.1.5_cards.png "Image of using crablit in alacritty terminal on Arch GNU/Linux"
