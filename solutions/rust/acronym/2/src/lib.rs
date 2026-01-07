pub fn abbreviate(phrase: &str) -> String {
    //todo!("Given the phrase '{phrase}', return its acronym");
    phrase.split(|c: char| !c.is_alphabetic() && c != '\'' && !c.is_uppercase())
        .filter(|word| !word.is_empty())
        .flat_map(|word| {
            word.chars().take(1).chain(
                word.chars()
                    .skip_while(|c| c.is_uppercase())
                    .filter(|c| c.is_uppercase()),
            )
        })
        .map(|c| c.to_ascii_uppercase())
        .collect()
        
}
