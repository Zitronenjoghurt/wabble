use crate::{WabbleDictionary, WabbleDictionaryEntry};
use serde::Deserialize;

const IGNORED_SENSE_TAGS: &[&str] = &["obsolete"];

#[derive(Debug, Deserialize)]
pub struct KaikkiEntry {
    pub pos: String,
    pub word: String,
    #[serde(default)]
    pub senses: Vec<KaikkiSense>,
}

impl KaikkiEntry {
    pub fn add_to_dictionary(&self, dictionary: &mut WabbleDictionary) {
        let Some(sense) = self.most_relevant_sense() else {
            return;
        };

        if !dictionary.has_word(&self.word) {
            dictionary.add_entry(&self.word, WabbleDictionaryEntry::default());
        }

        let Some(entry) = dictionary.get_mut(&self.word) else {
            return;
        };

        for gloss in sense.glosses.iter() {
            entry.add_definition(&self.pos, gloss);
        }
    }

    fn most_relevant_sense(&self) -> Option<&KaikkiSense> {
        if self.senses.is_empty() {
            return None;
        };

        'outer: for sense in self.senses.iter() {
            if sense.glosses.is_empty() {
                continue;
            };

            for tag in sense.tags.iter() {
                if IGNORED_SENSE_TAGS.contains(&tag.as_ref()) {
                    continue 'outer;
                }
            }

            return Some(sense);
        }

        None
    }
}

#[derive(Debug, Deserialize)]
pub struct KaikkiSense {
    #[serde(default)]
    pub glosses: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}
