/// Determine whether a sentence is a pangram.
use std::collections::HashSet;
pub fn is_pangram(sentence: &str) -> bool {
    if sentence.len()>=26 {
        let mut result = HashSet::new();
        for c in sentence.to_lowercase().trim().chars() {
          if c.is_alphabetic() && !result.contains(&c) {
            result.insert(c);
          }  
        }
        return result.len() == 26;
    }  
    return  false;
}
