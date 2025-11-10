use crate::parse::error::{ParseError, ParseResult};
use crate::parse::kaikki::KaikkiEntry;
use crate::WabbleDictionary;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use wabble_core::types::language::Language;

pub fn parse_dictionary(resources: &Path, language: Language) -> ParseResult<WabbleDictionary> {
    let word_whitelist = word_whitelist(resources, language)?;
    let kaikki_path =
        kaikki_path(resources, language)?.ok_or(ParseError::NoKaikkiFile(language))?;

    let kaikki_file = std::fs::File::open(&kaikki_path)?;
    let file_size = kaikki_file.metadata()?.len();
    let buf_reader = BufReader::new(kaikki_file);

    let pb = ProgressBar::new(file_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_message(format!("Processing {:?}", language));

    let mut dictionary = WabbleDictionary::new();
    let mut processed = 0;
    let mut accepted = 0;
    let mut bytes_read = 0u64;

    for line in buf_reader.lines() {
        let line = line?;
        bytes_read += line.len() as u64 + 1; // +1 for newline
        pb.set_position(bytes_read);

        let kaikki_entry: KaikkiEntry = serde_json::from_str(&line)?;
        processed += 1;

        if word_whitelist.contains(&kaikki_entry.word) {
            kaikki_entry.add_to_dictionary(&mut dictionary);
            accepted += 1;
        }
    }

    pb.finish_and_clear();
    println!(
        "{:?}: {} processed, {} accepted",
        language, processed, accepted
    );
    Ok(dictionary)
}

fn kaikki_path(resources: &Path, language: Language) -> ParseResult<Option<PathBuf>> {
    let language_name = match language {
        Language::English => "English",
        Language::German => "Deutsch",
    };

    let pattern = kaikki_regex(language_name);
    let re = Regex::new(&pattern)?;

    Ok(std::fs::read_dir(resources)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .find(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| re.is_match(name))
                .unwrap_or(false)
        }))
}

fn kaikki_regex(language_name: &str) -> String {
    format!(r"^kaikki\.org-dictionary-{language_name}\.jsonl$")
}

fn word_whitelist(resources: &Path, language: Language) -> ParseResult<HashSet<String>> {
    match language {
        Language::English => word_whitelist_txt(&resources.join("english_whitelist.txt")),
        Language::German => word_whitelist_txt(&resources.join("german_whitelist.txt")),
    }
}

fn word_whitelist_txt(path: &Path) -> ParseResult<HashSet<String>> {
    let whitelist = std::fs::read_to_string(path)?;
    let whitelist = whitelist
        .lines()
        .map(|line| line.trim().to_ascii_lowercase().to_string())
        .collect();
    Ok(whitelist)
}
