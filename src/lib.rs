extern crate lazy_static;

pub mod dictionary;

#[derive(Debug)]
pub struct Word {
    pub word: &'static str,
    // TODO: Turn the .0 into an enum.
    pub definitions: &'static [(&'static str, &'static str)],
}

// DATABASE
impl Word {
    pub fn new(word: &'static str, definitions: &'static [(&'static str, &'static str)]) -> Self {
        Word {
            word: word,
            definitions: definitions,
        }
    }
}
