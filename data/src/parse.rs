use crate::parse::dictionary::parse_dictionary;
use crate::parse::error::ParseResult;
use crate::WabbleData;
use std::path::Path;
use wabble_core::types::language::Language;

pub mod dictionary;
pub mod error;
mod kaikki;

pub fn parse_data(resources: &Path) -> ParseResult<WabbleData> {
    let english_dictionary = parse_dictionary(resources, Language::English)?;
    let german_dictionary = parse_dictionary(resources, Language::German)?;

    let mut data = WabbleData::default();
    data.dictionaries
        .insert(Language::English, english_dictionary);
    data.dictionaries
        .insert(Language::German, german_dictionary);

    Ok(data)
}
