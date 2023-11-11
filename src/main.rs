use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    crablit::start(&args);
    // let (delim, p) = nice_args(args);
    // let mut v = cards::init(Path::new("big.txt"), ';', 4);
    // while !v.is_empty() {
    //     v.shuffle(&mut thread_rng());
    //     v = cards::question(v);
    // }
}
