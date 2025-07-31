use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn read_word_list<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file).lines();

    let mut result = Vec::new();
    for line in buf {
        if let Ok(word) = line {
            let word_list = word.split(',');
            for word in word_list {
                let word = word.trim();
                if word.len() > 0 {
                    result.push(word.to_string());
                }
            }
        }
    }

    Ok(result)
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
}
