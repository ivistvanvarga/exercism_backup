use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    //todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let mut result: HashSet<&'a str> = HashSet::new();
    let l_word=word.to_lowercase();
    let mut word_v=l_word.chars().collect::<Vec<char>>();
    word_v.sort();
    
    let word_n= word_v.iter().collect::<String>();
        
    for t in possible_anagrams {
        let mut is_anagram= true;
        let anagram =t.to_lowercase();
        let mut anagram_v = anagram.chars().collect::<Vec<char>>();
        anagram_v.sort();
        let anagram_n= anagram_v.iter().collect::<String>();
                
            if anagram_n != word_n {
                is_anagram = false;
            }
        
        if anagram.len() == word.len() && !anagram.contains(&l_word) && is_anagram{
            result.insert(t);
        }
    }
    return result
}
