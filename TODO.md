# TODO:

- [x] basics
    + [x] initialization from file <- `std::fs`
    + [x] basic matching every term <- vector of either `Cards` or `Verbs`
    + [x] spaced repetition <- iterating over vector, pushing back not guessed ones, they get questioned later, shuffled
    + [x] end-user dealing with typos <- typo
    + [x] hints <- hint
    + [x] colored output <- `colored` from cargo

- [ ] further development
    + [x] make exit work just like that <- done using `exit()`
    + [x] print how much to go out of how many <- kinda implemented using `v.len()`
    + [ ] don't write almost the same functions twice
    + [ ] levelled knowledge
    + [ ] flashcards
    + [ ] add keywords skip and repeat <- done: skip: `continue;`, repeat is hard because of &-s and borrowing
    + [ ] make hint smarter: output right amount of '_'-s even for special (more byte) characers more hint
    + [ ] have organized, formatted output
    + [ ] starring words that need learning
    + [ ] `rustyline` for editable input
    + [ ] have option to ask for term or definition
    + [ ] docs

- [ ] nice-to-haves
    + [ ] filtering what to learn
    + [ ] taking cli arguments
        * [x] file name
        * [ ] delimiter
        * [ ] mode
        * [ ] title
        * [ ] author
        * [ ] languages
        * [ ] verbosity
        * [ ] help
        * [ ] other great stuff
    + [ ] fetching definitions from 
        * [Dudenswissensnetz](duden.de) 
        * [verbformen](verbformen.de)
        * [cambrigde online dictionary](dictionary.cambrigde.org)
        * lots of other stuff
        * using [deepl](deepl.com)

    + [ ] write good manual
    + [ ] being able to determine every property itself
    + [ ] everything knowt or quizlet has
    + [x] being able to learn verbs with their forms from tables
    + [ ] boxed layout
    + [ ] fuzzy matching words
    + [ ] tests: like in school
    + [ ] remembering session
    + [ ] very nice statistics

- [ ] development
    + [ ] tests: being cross-platform
    + [ ] being stupid-proof == no bugs
    + [ ] non-spagethi code with 
        * [x] `mod`s 
        * [ ] `enum`s 
        * [ ] all kinds of crazy stuff  

    + [ ] [github](https://github.com/JeromeSchmied) repo, or some github alternative 
    + [ ] being able to share stuff
    + [ ] being able to create flashcard sets in app
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

