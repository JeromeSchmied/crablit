pub fn user_input(qst: &str) -> String {
    println!("{qst}");
    let mut babineni = String::new();
    std::io::stdin()
        .read_line(&mut babineni)
        .expect("what is goink on?");
    if babineni.ends_with('\n') {
        babineni.pop();
    }
    babineni
}
