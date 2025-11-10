use bincode::{Decode, Encode};
use std::collections::HashMap;
use wabble_core::types::language::Language;

#[cfg(feature = "parse")]
pub mod parse;

#[derive(Debug, Default, Encode, Decode)]
pub struct WabbleData {
    pub dictionaries: HashMap<Language, WabbleDictionary>,
}

#[derive(Debug, Default, Encode, Decode)]
pub struct WabbleDictionary(HashMap<String, WabbleDictionaryEntry>);

impl WabbleDictionary {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn has_word(&self, word: impl AsRef<str>) -> bool {
        self.0.contains_key(word.as_ref())
    }

    pub fn add_entry(&mut self, word: impl AsRef<str>, entry: WabbleDictionaryEntry) {
        self.0.insert(word.as_ref().to_string(), entry);
    }

    pub fn get(&self, word: impl AsRef<str>) -> Option<&WabbleDictionaryEntry> {
        self.0.get(word.as_ref())
    }

    pub fn get_mut(&mut self, word: impl AsRef<str>) -> Option<&mut WabbleDictionaryEntry> {
        self.0.get_mut(word.as_ref())
    }
}

#[derive(Debug, Default, Encode, Decode)]
pub struct WabbleDictionaryEntry {
    /// Definitions for each part of speech.
    pub definitions: HashMap<String, Vec<String>>,
}

impl WabbleDictionaryEntry {
    pub fn add_definition(&mut self, part_of_speech: impl AsRef<str>, definition: impl AsRef<str>) {
        self.definitions
            .entry(part_of_speech.as_ref().to_string())
            .or_default()
            .push(definition.as_ref().to_string());
    }
}
