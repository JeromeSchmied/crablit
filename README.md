# [learnit](https://github.com/JeromeSchmied/learnit) (some better name needed!)

## [anki](https://ankiweb.net), [quizlet](https://quizlet.com) and [knowt](https://knowt.com) inspired app, just right in the terminal

## how it works
- it takes a source file: .tsv, .csv or .txt. See [examples](https://github.com/JeromeSchmied/learnit/tree/main/examples)
- stores it in a vector
- asks them until you know all well (currently only till you guess them right once)

## why it's better than the others?

|                 | quizlet     | knowt      | learnit                    |
|---------------- | ----------- | ---------- | -------------------------- |
| open-source     | no          | no         | of course!                 |
| ad-free         | nope        | nope       | 100%                       |
| totally free    | not really  | not really | Yes, and it always will be |
| speed out of 10 | 4           | 2          | 10                         |
| offline version | paid        | no         | cross-platform, fast, TUI: coming soon  |

## installing:

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
cargo run -- examples/verbs.tsv
```

## to help, see [todos](TODO.md)

## alternatives: 
- [speki](https://crates.io/crates/speki): only flashcards, rust
- [vocage](https://crates.io/crates/vocage): only flashcards, rust
- [fla.sh](https://github.com/tallguyjenks/fla.sh): only flashcards, bash
- [hascard](https://github.com/Yvee1/hascard): haskell
- [exhaust](https://github.com/heyrict/exhaust): rust
