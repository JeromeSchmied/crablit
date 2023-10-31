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
