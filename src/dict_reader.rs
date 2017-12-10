use std::fs::File;
use std::io::{BufRead, BufReader};
use util;
use config;

pub struct DictReader {
    filepath: String,
}

impl DictReader {
    pub fn new(filepath: &str) -> DictReader {
        DictReader {
            filepath: String::from(filepath),
        }
    }

    pub fn get_wordvec(&self) -> Vec<String> {
        let filepath = &self.filepath;

        let file = File::open(filepath).unwrap_or_else(|_| {
            util::exit_with_message(&format!("{}: {}", config::WORDLIST_READ_ERR, filepath))
        });

        let reader = BufReader::new(file);

        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

        lines
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn my_test() {}

}
