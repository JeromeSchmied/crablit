# learnit --> some better name needed

## [quizlet](https://quizlet.com) and [knowt](https://knowt.com) inspired kind of thing just the right way (in the terminal)

we need a source: text file: \<`term`\>\<`delimiter`\>\<`definition`\>\n
we need a struct: `Bela`  to put terms and definitions: `terms: String, defs: String`
than we need a vector like: `let v: Vec<Bela>`
and we show `term`, take input, if it's the same as `definition`, goes to slightly_knows
else it goes to `doesnt_know`. It loops and after there is nothing in `doesnt_know`,
it keeps asking from `slightly_knows`, and so on.

## why it's better than the others?

|                 | quizlet     | knowt      | learnit                    |
|---------------- | ----------- | ---------- | -------------------------- |
| open-source     | no          | no         | of course!                 |
| ad-free         | nope        | nope       | 100%                       |
| totally free    | not really  | not really | Yes, and it always will be |
| speed out of 10 | 4           | 2          | 10                         |
| offline version | paid        | no         | cross-platform, fast, TUI  |


TODO:

for myself:
- [x] make typo work
- [x] make hint work
- [ ] make exit work just like that
- [ ] don't write things twice
- [ ] have organized output

- [ ] basics
    + [x] initialization of data type from source file
    + [x] basic matching every term
    + [x] spaced repetition
    + [x] end-user dealing with typos
    + [x] hints
    + [x] colored output

- [ ] further development
    + [ ] flashcards
    + [ ] starring words that need learning
    + [ ] levelled knowledge

- [ ] nice-to-haves
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

```rust
struct Bela {
    term: String,
    def: String,
    // how well u know it
    level: u8,
}

fn main(){
    println!("flashcards in the terminal");
}
```
