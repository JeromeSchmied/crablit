# [Crablit](https://github.com/JeromeSchmied/crablit): Learning app inspired by [anki](https://ankiweb.net), [quizlet](https://quizlet.com) and [knowt](https://knowt.com), but in the terminal.

[![Crates.io Version](https://img.shields.io/crates/v/crablit)](https://crates.io/crates/crablit)
[![Crates.io License](https://img.shields.io/crates/l/crablit)](https://choosealicense.com/licenses/mit/)
[![dependency status](https://deps.rs/crate/crablit/0.1.6/status.svg)](https://deps.rs/crate/crablit/0.1.6)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/jeromeschmied/crablit/rust.yml)](https://github.com/jeromeschmied/crablit/actions)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/w/jeromeschmied/crablit)](https://github.com/jeromeschmied/crablit/commits)
[![GitHub issues](https://img.shields.io/github/issues/jeromeschmied/crablit)](https://github.com/jeromeschmied/crablit/issues)
[![GitHub top language](https://img.shields.io/github/languages/top/jeromeschmied/crablit)](https://github.com/jeromeschmied/crablit/)

> ## IMPORTANT!
>
> Crablit is still in beta, functionality is not tested thoroughly.

## Table of Contents (ToC)

<!--toc:start-->

-   [Features](#features)
-   [Creating files to learn](#creating-files-to-learn)
-   [Installing](#installing)
-   [Usage](#usage)
-   [How it works](#how-it-works)
-   [Notes](#notes)
-   [Alternatives](#alternatives)
<!--toc:end-->

## Features

-   _open-source_, MIT licensed
-   _Cross-platform_: works on Windows, MacOS, but with GNU/Linux 1st philosophy.
-   _Fast_: initialization of **20000000** cards (574MB) takes about **4.3s** with a maximum of 2.4GB RAM usage.
-   Written answers.

## Creating files to learn

-   _Easy deck making_, in your favourite editor:
    -   Simple `.txt`, `.csv` or `.tsv` file.
    -   Card: `<term><delimiter><definition>`. eg: `"to learn;lernen"`.
    -   File of Cards with the same delimiter in every line.
    -   Supported delimiters are: `';', '|', '    '(tab), '=', ':'`.
    -   Lines starting with `#` are considered to be comments.
    -   Extra newlines, indent cause no problem.
    -   Extra spaces around delimiter cause no problem, such as: `hooray | booyah!`.
    -   For instance:

```text
# example file that can be parsed by crablit

    # special soup!
    soup | Suppe

fast | schnell
alpaca | das Alpaka
proud | stolz
# I don't know what to write next. | Ich weiß nicht was...
to pour | gießen
```

-   Mode for _Verb-learning_: if you need to learn lots of verbforms, like:

```text
# [crablit]
# mode = "verbs"
# delim = '	'

# Verben Tabelle von DaF leicht 1,2
# it's hungarian but that really doesn't matter

# INFINI	DRITTE	PRÄTER	PERFEKT 	MEANING 	KOMMENT
atmen	    atmet	atmete	h. geatmet	lélegzik	Er hat ruhig geatmet.
baden	    badet	badete	h. gebadet	fürdik  	Das Mädchen duscht den Hund.
bauen	    baut	baute	h. gebaut	épít    	Haus bauen
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

## Installing

### MSRV: Minimum Supported Rust Version is `1.74.1`

0. **_Easiest, recommended:_** Go to releases page and see instructions built by [cargo-dist](https://opensource.axo.dev/cargo-dist/).

or:

-   [Install Rust](https://www.rust-lang.org/tools/install), in case you don't have it.
-   Open a [terminal](https://github.com/cdleon/awesome-terminals) and do one of the following install methods:

1. From [crates.io](https://crates.io/crates/crablit):

```shell
cargo install crablit --locked
```

3. Download source from github:

```shell
# if on windows: curl.exe
curl -L "https://github.com/JeromeSchmied/crablit/archive/main.tar.gz" | tar -xzf -
# go to the directory where it's been cloned
cd crablit-main
# install it
cargo install --path .
```

3. Or you may download source with [git](https://git-scm.com/downloads):

```shell
# once you have git, clone the repo to have it locally
git clone --depth=1 https://github.com/JeromeSchmied/crablit.git
# go to the directory where it's been cloned
cd crablit
# install it
cargo install --path .
```

## Usage

-   Run `crablit --help` to see help.
-   Type the definition of the questioned word/phrase.
-   To see hint: `:help`, `:hint` or `:h`.
-   If you mistyped it, type: `:typo`.
-   To skip: `:skip`.
-   To quit: `quit` or `exit` or `:q`.
-   To save progress: `:w` or `:write` or `:save`, to continue, you can open the file just like before. It's not yet stable on Windows.
-   To save, then quit: `:wq`.
-   To go to the ones not guessed correctly and ignore the other ones: `:revise`.
-   To see flashcard-like stuff: `:f` or `:flash`. Yet untested.
-   To check status: `:n`, `:num` or `:togo`.
-   To edit previously typed guess, press arrow-up. Doesn't always work.
-   If `NO_COLOR=1`, coloring is disabled, thanks to [colored](https://crates.io/crates/colored).

## How it works

![Sample][1]

-   See [images for more](examples/img)!
-   It takes a source text file with deck of cards: .tsv, .csv or .txt. See [examples](https://github.com/JeromeSchmied/crablit/tree/main/examples).
-   Stores them in a vector.
-   Asks them until you know all well (currently only till you guess them right once).

## Notes

-   I'm only learning Rust at the moment, so code quality might not be outstanding.
-   Flashcards, TUI, GUI, Website are coming, but it takes time.
-   Any bugs, questions, feature requests shall be reported to [github](https://github.com/JeromeSchmied/crablit/issues), or by [email](mailto:iitsnotme214@proton.me).
-   To help with development, see [todos](TODO.md).

## Alternatives

-   [speki](https://crates.io/crates/speki): pretty great, but only flashcards, huge download size, tui, rust, unmaintained?
-   [vocage](https://crates.io/crates/vocage): only flashcards, tui, rust, unmaintained?
-   [flcard](https://crates.io/crates/flcard): only flashcards, very simple, rust, unmaintained?
-   [fla.sh](https://github.com/tallguyjenks/fla.sh): only flashcards, bash
-   [hascard](https://github.com/Yvee1/hascard): only flashcards, haskell
-   [exhaust](https://github.com/heyrict/exhaust): I couldn't get it to work, rust, unmaintained?

[1]: examples/img/v0.1.5_cards.png "Image of using crablit in Alacritty terminal on Arch GNU/Linux"
