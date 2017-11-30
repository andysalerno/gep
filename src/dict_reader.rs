use std::fs::File;
use std::io::{BufRead, BufReader};

const MIN_ALLOWED_SIZE: usize = 10_000;

pub struct DictReader {
    filepath: String,
    wordvec: Vec<String>,
}

impl DictReader {
    pub fn new(filepath: String) -> DictReader {
        let wordvec = DictReader::dict_to_wordvec(&filepath);

        if wordvec.len() < MIN_ALLOWED_SIZE {
            panic!("The minimum allowed dictionary length is: {}. Cannot use dict '{}' with length: {}",
            MIN_ALLOWED_SIZE, filepath, wordvec.len());
        }

        DictReader {
            filepath: filepath,
            wordvec: wordvec,
        }
    }

    pub fn len(&self) -> usize {
        self.wordvec.len()
    }

    pub fn get_nth_word(&self, n: usize) -> &str {
        &self.wordvec[n]
    }

    fn dict_to_wordvec(filepath: &str) -> Vec<String> {
        let file =
            File::open(filepath).expect(&format!("couldn't open dictionary file with path: {}", filepath));

        let reader = BufReader::new(file);

        let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();

        lines
    }
}