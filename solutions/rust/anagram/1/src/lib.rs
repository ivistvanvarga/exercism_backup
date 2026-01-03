use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    //todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let mut result: HashSet<&'a str> = HashSet::new();
    let l_word=word.to_lowercase();
    let word_h: HashSet<char> = HashSet::from_iter(l_word.chars());
        
    for t in possible_anagrams {
        let mut is_anagram= true;
        let anagram =t.to_lowercase();
        let mut anagram_h: HashSet<char> = HashSet::new();
                
        for _a_c in anagram.chars() {
            
            
            if !word_h.contains(&_a_c) || anagram_h.contains(&_a_c) {
                is_anagram = false;
                break;
            }
            anagram_h.insert(_a_c);

        }

        if anagram.len() == word.len() && !anagram.contains(&l_word) && is_anagram{
            result.insert(t);
        }
    }
    return result
}
