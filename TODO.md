# TODO:

## basics

-   [x] initialization from file <- `std::fs`
-   [x] basic matching every term <- vector of either `Cards` or `Verbs`
-   [x] spaced repetition <- iterating over vector, pushing back not guessed ones, they get questioned later, shuffled
-   [x] end-user dealing with typos <- :typo
-   [x] hints <- :hint
-   [x] colored output <- [colored](https://crates.io/crates/colored) from crates.io

## further development

-   [x] make exit work just like that <- done using `exit()`
-   [x] print number of cards to learn <- kinda implemented using `v.len()`
-   [x] add keyword skip <- done: `continue;`
-   [x] `rustyline` for editable input <- done, followed docs
-   [x] `clap` <- done, usage seen from tplay, flcard
-   [x] output the right amount of '\_'-s even for special (more byte) characers <- done: `s.chars().count()`
-   [x] have organized, formatted output <- done: using nice charecters, colors
-   [x] save progress <- done using dirs crate, fs::read,write
-   [ ] make progress saving stable
-   [ ] autocompletion of commands: `:<command>`
-   [ ] levelled knowledge
-   [ ] starring words that need learning, high level
-   [ ] use a printer String, always delete old questions: pretty-print
-   [ ] flashcards
-   [ ] find duplicates and deal with 'em

## nice-to-haves

-   [ ] if hint typed more than once, show more help
-   [ ] [tui](https://crates.io/crates/ratatui)
-   [ ] filtering what to learn, like unusual verbforms
-   [ ] taking cli arguments: [clap](https://crates.io/crates/clap)
    -   [x] file name
    -   [x] delimiter
    -   [x] mode
    -   [x] swapping term and definition fully or partially
    -   [x] help
    -   [x] shuffling
    -   [ ] verbosity: clap-verbosity-flag
    -   [ ] other great stuff
-   [ ] fetching definitions from

    -   [ ] anki decks
    -   [ ] quizlet decks
    -   [ ] knowt decks
    -   [ ] [chatgpt](https://chat.openai.com) just because everyone is crazy about it
    -   [ ] [Dudenswissensnetz](https://duden.de)
    -   [ ] [verbformen](https://verbformen.de)
    -   [ ] [oxford english dictionary](https://oed.com/dictionary)
    -   [ ] lots of other stuff
    -   [ ] using [deepl](https://deepl.com)

-   [ ] write docs, man
-   [x] being able to determine every property itself <- kinda?
-   [ ] everything knowt or quizlet has
-   [x] being able to learn verbs with their forms from tables
-   [ ] fuzzy matching words
-   [ ] tests: like in school
-   [ ] very nice statistics, plots

## development

-   [x] clean main.rs
-   [x] don't write almost the same functions twice, use generics and all <- init(), quest() now uses traits, generics
-   [x] basic tests: being cross-platform, `lib` tests
-   [ ] tests: assert_cmd
-   [ ] being stupid-proof == no bugs
-   [ ] non-spagethi code with

    -   [x] `mod`s
    -   [x] `enum`s
    -   [x] `generics`
    -   [x] `trait`s
    -   [x] `Result`s
    -   [ ] all kinds of crazy stuff

-   [x] [github repo](https://github.com/JeromeSchmied/crablit), or some github alternative
-   [x] [crates](https://crates.io)
-   [ ] website: crablit.com or crablit.io or crab.lit
-   [ ] being able to share decks
-   [ ] being able to create flashcard decks in app
-   [ ] clean API
-   [ ] template files for creating great flashcards with
    -   [x] delimiter
    -   [x] mode
    -   [ ] title
    -   [ ] author
    -   [ ] languages
    -   [ ] other great stuff
-   [ ] community
-   [ ] gui interface: dioxus?
    -   [ ] webapp for learning
    -   [ ] desktop apps
    -   [ ] mobile
