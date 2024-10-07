pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .flat_map(|word| {
            word.chars().take(1).chain(
                word.chars()
                    .skip_while(|c| c.is_uppercase())
                    .filter(|c| c.is_uppercase()),
            )
        })
        .collect::<String>()
        .to_uppercase()
}

fn main() {
    let phrase = "HyperText Markup Language";
    let abbreviation = abbreviate(phrase);
    println!("The abbreviation for '{}' is '{}'", phrase, abbreviation);
}
