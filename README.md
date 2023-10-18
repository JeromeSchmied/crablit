# learnit --> some better name needed

## [quizlet](https://quizlet.com) and [knowt](https://knowt.com) inspired kind of thing just the good way (in the terminal)

we need a source: text file: \<`term`\>\<`delimiter`\>\<`definition`\>\n
we need a struct: `Bela`  to put terms and definitions: `terms: String, defs: String`
than we need a vector like: `let v: Vec<Bela>`
and we show `term`, take input, if it's the same as `definition`, goes to slightly_knows
else it goes to `doesnt_know`. It loops and after there is nothing in `doesnt_know`,
it keeps asking from `slightly_knows`, and so on.

TODO:

- [ ] basic usage
    + [x] initialization of data type from source file
    + [x] basic matching every term
    + [x] spaced repetition
    + [ ] starring words that need learning
    + [ ] end-user dealing with typos
    + [ ] flashcards
    + [ ] levelled knowledge
    + [x] colored output
    + [x] taking cli arguments
- [ ] development
    + [ ] being stupid-proof
    + [ ] non-spagethi code with 
        * [x] `mod`s 
        * [ ] `enum`s 
        * [ ] all kinds of crazy stuff  
    + [ ] [github](https://github.com/JeromeSchmied) repo, or some github alternative 
    + [ ] community
    + [ ] being able to share stuff
    + [ ] being able to create flashcard sets in app
- [ ] nice-to-haves
    + [x] being able to learn verbs with their forms from tables
    + [ ] boxed layout
    + [ ] template files for creating great flashcards with delimiter, type, title, author and interesting stuff
    + [ ] fuzzy matching words
    + [ ] exam tests
    + [ ] remembering session
    + [ ] very nice statistics

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
