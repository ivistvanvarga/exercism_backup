pub fn abbreviate(phrase: &str) -> String {
    //todo!("Given the phrase '{phrase}', return its acronym");
    phrase.split(|c: char| !c.is_alphabetic() && c != '\'' && !c.is_uppercase())
        .filter(|word| !word.is_empty())
        .map(|word| word.chars().next().unwrap().to_ascii_uppercase())
        .collect()
        
}
