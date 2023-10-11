# learnit --> some better name needed

## [quizlet](https://quizlet.com) and [knowt](https://knowt.com) inspired kind of thing just the good way (in the terminal)

we need a source: text file: \<`term`\>\<`delimiter`\>\<`definition`\>\n
we need a struct: `Bela`  to put terms and definitions: `terms: String, defs: String`
than we need a vector like: `let v: Vec<Bela>`
and we show `term`, take input, if it's the same as `definition`, goes to slightly_knows
else it goes to `doesnt_know`. It loops and after there is nothing in `doesnt_know`,
it keeps asking from `slightly_knows`, and so on.

TODO:

- [ ] initialization of data type from source file
- [ ] basic matching every term
- [ ] spaced repetition
- [ ] starring words that need learning
- [ ] colored output
- [ ] no-spagethi code with `mod`s and `enum`s and all kinds of crazy stuff  
- [ ] flashcards
- [ ] template files for creating great flashcards with title, author and interesting stuff
- [ ] fuzzy matching words
- [ ] exam tests
- [ ] [github](https://github.com/JeromeSchmied) repo, or some github alternative 
- [ ] remembering session
- [ ] community
- [ ] being able to share stuff
- [ ] being able to create flashcard sets in app
- [ ] very nice statistics

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
