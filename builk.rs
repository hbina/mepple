extern crate sqlite;

use std::io::prelude::*;

const SQLITE_DATABASE: &str = "data/dictionary.db";

#[derive(Debug)]
struct Word {
    word: String,
    // TODO: The first component should be an enum.
    // TODO: Process definition a little more.
    definitions: Vec<(String, String)>,
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Word {{\n\tword: \"{}\",\n\tdefinitions: &{:?},\n}}",
            self.word, self.definitions
        )
    }
}

impl Word {
    pub fn new(word: String, word_type: String, definition: String) -> Self {
        Word {
            word: word,
            definitions: vec![(word_type, definition)],
        }
    }

    pub fn add(&mut self, word_type: String, definition: String) -> &mut Self {
        self.definitions.push((word_type, definition));
        self
    }
}

// TODO: Remove all this clones later...

fn main() {
    // Notify cargo to rebuild everytime the database changes.
    println!("cargo:rerun-if-changed={}", SQLITE_DATABASE);

    // Open SQLITE file.
    let connection = sqlite::open(SQLITE_DATABASE).unwrap();
    let mut statement = connection.prepare("SELECT * FROM entries").unwrap();

    // Read and convert SQLITE data into memory.
    let mut words = std::collections::HashMap::<String, Word>::with_capacity(10_000);
    while let sqlite::State::Row = statement.next().unwrap() {
        let word = statement.read::<String>(0).unwrap();
        let word_type = statement.read::<String>(1).unwrap();
        let definition = statement.read::<String>(2).unwrap();

        let entry = words.entry(word.clone()).or_insert(Word::new(
            word.clone(),
            word_type.clone(),
            definition.clone(),
        ));
        entry.add(word_type.clone(), definition.clone());
    }

    // Write data to file.
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .open("src/dictionary.rs")
        .unwrap();
    let contents = format!(
        r#"use crate::Word;

    pub const WORDS: &'static [Word] = &[
{}
    ];
"#,
        words
            .iter()
            .inspect(|x| println!("x:{}", x.1))
            .fold(String::with_capacity(50_000), |acc, x| {
                format!("{}{},", acc, x.1)
            })
    );
    file.write_all(contents.as_bytes()).unwrap();
}
