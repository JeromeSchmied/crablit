# TODO:

## basics
- [x] initialization from file <- `std::fs`
- [x] basic matching every term <- vector of either `Cards` or `Verbs`
- [x] spaced repetition <- iterating over vector, pushing back not guessed ones, they get questioned later, shuffled
- [x] end-user dealing with typos <- typo
- [x] hints <- hint
- [x] colored output <- [colored](https://crates.io/crates/colored) from cargo

## further development
- [x] make exit work just like that <- done using `exit()`
- [x] print how much to go out of how many <- kinda implemented using `v.len()`
- [ ] levelled knowledge
- [ ] flashcards
- [ ] add keywords skip and repeat <- done: skip: `continue;`, repeat is hard because of &-s and borrowing
- [ ] make hint smarter: output right amount of '_'-s even for special (more byte) characers more hint
- [ ] `rustyline` for editable input
- [ ] find duplicates and deal with 'em
- [ ] starring words that need learning
- [ ] don't write almost the same functions twice
- [ ] save progress
- [ ] docs

## nice-to-haves
- [ ] have organized, formatted output
- [ ] [tui](https://crates.io/crates/ratatui) 
- [ ] have option to ask for term or definition
- [ ] filtering what to learn
- [ ] taking cli arguments: [clap](https://crates.io/crates/clap) 
    + [x] file name
    + [ ] delimiter
    + [ ] mode
    + [ ] title
    + [ ] author
    + [ ] languages
    + [ ] verbosity
    + [ ] help
    + [ ] other great stuff
- [ ] fetching definitions from 
    + anki decks
    + quizlet decks
    + knowt decks
    + [chatgpt](https://chat.openai.com) because everyone is crazy about it
    + [Dudenswissensnetz](https://duden.de) 
    + [verbformen](https://verbformen.de)
    + [cambrigde online dictionary](https://dictionary.cambrigde.org)
    + lots of other stuff
    + using [deepl](https://deepl.com)

- [ ] write good manual
- [ ] being able to determine every property itself
- [ ] everything knowt or quizlet has
- [x] being able to learn verbs with their forms from tables
- [ ] boxed layout
- [ ] fuzzy matching words
- [ ] tests: like in school
- [ ] remembering session
- [ ] very nice statistics

## development
+ [ ] tests: being cross-platform
+ [ ] being stupid-proof == no bugs
+ [ ] non-spagethi code with 
    * [x] `mod`s 
    * [ ] `enum`s 
    * [ ] all kinds of crazy stuff  

+ [x] [github](https://github.com/JeromeSchmied/learnit) repo, or some github alternative 
+ [ ] [crates](https://crates.io)
+ [ ] being able to share decks
+ [ ] being able to create flashcard decks in app
+ [ ] clean API
+ [ ] template files for creating great flashcards with
    * [x] delimiter
    * [x] mode
    * [ ] title
    * [ ] author
    * [ ] languages
    * [ ] other great stuff
+ [ ] community
+ [ ] gui interface
    * [ ] desktop apps
    * [ ] website for learning
    * [ ] mobile
