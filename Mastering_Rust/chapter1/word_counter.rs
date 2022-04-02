use std::env;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

#[derive(Debug)]
struct WordCounter(HashMap<String, u64>);

impl WordCounter {
    fn new() -> WordCounter {
        WordCounter(HashMap::new())
    }

    fn increment(&mut self, word: &str) {
        let key = word.to_string();
        let count = self.0.entry(key).or_insert(0);
        *count += 1;
    }

    fn display(&self, filter: u64) {
        let mut temp: Vec<_> = self.0.iter().collect();
        temp.sort_by_key(|a| a.1);
        for (key, value) in temp.iter() {
            if value > &&filter {
                println!("{}: {}", key, value);
            }
        }
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let filename = &arguments[1];

    println!("Processing file: {}", filename);

    let file = File::open(filename).expect("Could not open file");

    let reader = BufReader::new(file);

    let mut word_counter = WordCounter::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let words = line.split(" ");

        for word in words {
            if word == "" {
                continue
            } else {
                word_counter.increment(word);
            }
        }
    }

    word_counter.display(1);
//    word_counter.display();
}
