use regex::Regex;

use crate::utils::charsets::{UNICODE_LETTER_MARK_NUMBER_CHARSET, UNICODE_PUNCT_SYMBOL_CHARSET};

lazy_static::lazy_static! {
    // Generate string sets for regex
    static ref ALL_PUNCT: String = UNICODE_PUNCT_SYMBOL_CHARSET.iter().collect();
    static ref ALL_LETTER_MARK_NUMBER: String = UNICODE_LETTER_MARK_NUMBER_CHARSET.iter().collect();

    // Tokenizer regex
    pub static ref TOKENIZE_RE: Regex = Regex::new(&format!(
        "[{}]|[{}]+",
        regex::escape(&ALL_PUNCT),
        regex::escape(&ALL_LETTER_MARK_NUMBER),
    )).expect("Invalid regex");
}

/// Tokenizes a sentence into words based on punctuation and letter/number groups.
///
/// # Arguments
///
/// * `sentence` - The input sentence to tokenize.
///
/// # Returns
///
/// A vector of tokens as strings.
pub fn simple_word_tokenize(sentence: &str) -> Vec<String> {
    TOKENIZE_RE
        .find_iter(sentence)
        .map(|mat| {
            println!("tokenization");
            mat.as_str().to_string()
        })
        .collect()
}
