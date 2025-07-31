use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn read_word_list<P>(filename: P) -> io::Result<HashMap<String, Vec<String>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file).lines();

    let mut result = HashMap::new();
    for line in buf {
        if let Ok(word) = line {
            let word_list = word.split(',');
            for word in word_list {
                let conv_word = word.trim();
                let mut conv_word = conv_word.to_lowercase().chars().collect::<Vec<char>>();
                conv_word.sort();
                let conv_word: String = conv_word.iter().collect();
                if conv_word.len() > 0 {
                    result.entry(conv_word.clone()).or_insert(Vec::new()).push(word.to_string());
                }
            }
        }
    }

    Ok(result)
}

fn is_anagram(s1: &str, s2: &str) -> bool {
    let mut s1 = s1.chars().collect::<Vec<char>>();
    let mut s2 = s2.chars().collect::<Vec<char>>();
    s1.sort();
    s2.sort();
    s1 == s2
}

fn anagram(word: &str, word_list: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut word = word.to_lowercase().chars().collect::<Vec<char>>();
    word.sort();
    let word: String = word.iter().collect();
    if word_list.contains_key(&word) {
        println!("Find {word}");
        return word_list[&word].clone();
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_word_list() {
        let word_list = read_word_list("src/word_list.txt").unwrap();
        println!("{:?}", word_list);
        assert!(word_list.len() > 0);
    }

    #[test]
    fn test_is_anagram_simple() {
        assert!(is_anagram("listen", "silent"));
    }

    #[test]
    fn test_anagram() {
        let word_list = read_word_list("src/word_list.txt").unwrap();
        println!("{:?}", word_list);
        let anagrams = anagram("koli", &word_list);
        println!("{:?}", anagrams);
        let expected = vec!["kilo"];
        assert_eq!(anagrams, expected);
    }
}
