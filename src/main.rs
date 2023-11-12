use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, author, long_about = None)]
struct Args {
    /// Path of the file to learn.
    #[arg(required = true)]
    file: String,
    // /// Mode.
    // #[arg(short, long, default_value = "cards")]
    // mode: String,

    // /// Swap terms and definitions
    // #[arg(short, long, default_value_t = false)]
    // swap: bool,
}

fn main() {
    let path = Args::parse().file;
    // let mode = Args::parse().mode;
    let (mode, delim, n) = crablit::determine_properties(&path);
    crablit::start(mode, delim, n, path);
    // let (delim, p) = nice_args(args);
    // let mut v = cards::init(Path::new("big.txt"), ';', 4);
    // while !v.is_empty() {
    //     v.shuffle(&mut thread_rng());
    //     v = cards::question(v);
    // }
}
