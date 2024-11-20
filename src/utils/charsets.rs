use std::collections::{HashMap, HashSet};
use unicode_general_category::{get_general_category, GeneralCategory};

fn categorize() -> HashMap<GeneralCategory, Vec<char>> {
    let mut map: HashMap<_, Vec<char>> = HashMap::new();
    for ch in 0..=65535 {
        if let Some(ch) = char::from_u32(ch) {
            let cat = get_general_category(ch);
            if let Some(v) = map.get_mut(&cat) {
                v.push(ch);
            } else {
                map.insert(cat, vec![ch]);
            }
        }
    }
    map
}

lazy_static::lazy_static! {
    static ref ALL_UNICODE: HashMap<GeneralCategory, Vec<char>> = categorize();

    static ref UNICODE_PUNCT_CHARSET: Vec<char> = vec![
        GeneralCategory::DashPunctuation,
        GeneralCategory::OpenPunctuation,
        GeneralCategory::ClosePunctuation,
        GeneralCategory::FinalPunctuation,
        GeneralCategory::OtherPunctuation,
        GeneralCategory::InitialPunctuation,
        GeneralCategory::ConnectorPunctuation,
        ].iter()
        .flat_map(|cat| ALL_UNICODE.get(cat).unwrap().to_owned())
        .collect();

    static ref UNICODE_SYMBOL_CHARSET: Vec<char> = vec![
        GeneralCategory::MathSymbol,
        GeneralCategory::OtherSymbol,
        GeneralCategory::CurrencySymbol,
        GeneralCategory::ModifierSymbol,
        ].iter()
        .flat_map(|cat| ALL_UNICODE.get(cat).unwrap().to_owned())
        .collect();
    pub static ref UNICODE_PUNCT_SYMBOL_CHARSET: Vec<char> = [
        UNICODE_PUNCT_CHARSET.as_slice(),
        UNICODE_SYMBOL_CHARSET.as_slice(),
        ].concat();

    static ref UNICODE_LETTER_CHARSET: Vec<char> = vec![
        GeneralCategory::OtherLetter,
        GeneralCategory::ModifierLetter,
        GeneralCategory::LowercaseLetter,
        GeneralCategory::TitlecaseLetter,
        GeneralCategory::UppercaseLetter,
        ].iter()
        .flat_map(|cat| ALL_UNICODE.get(cat).unwrap().to_owned())
        .collect();
    static ref UNICODE_MARK_CHARSET: Vec<char> = vec![
        GeneralCategory::SpacingMark,
        GeneralCategory::EnclosingMark,
        GeneralCategory::NonspacingMark,
        ].iter()
        .flat_map(|cat| ALL_UNICODE.get(cat).unwrap().to_owned())
        .collect();
    static ref UNICODE_NUMBER_CHARSET: Vec<char> = vec![
        GeneralCategory::OtherNumber,
        GeneralCategory::LetterNumber,
        GeneralCategory::DecimalNumber,
        ].iter()
        .flat_map(|cat| ALL_UNICODE.get(cat).unwrap().to_owned())
        .collect();
    pub static ref UNICODE_LETTER_MARK_NUMBER_CHARSET: Vec<char> = [
        UNICODE_LETTER_CHARSET.as_slice(),
        UNICODE_MARK_CHARSET.as_slice(),
        ]
        .concat();

    static ref AR_LETTERS_CHARSET: HashSet<char> = "ءآأؤإئابةتثجحخدذرزسشصضطظعغـفقكلمنهويپچڤگ".chars().collect();
    static ref AR_DIAC_CHARSET: HashSet<char> = "ًٌٍَُِّْٰـ".chars().collect();
    pub static ref AR_CHARSET: HashSet<char> = AR_LETTERS_CHARSET.union(&AR_DIAC_CHARSET).cloned().collect();

    static ref BW_LETTERS_CHARSET: HashSet<char> = "$&'*<>ADEGHJPSTVYZ_bdfghjklmnpqrstvwxyz{|}".chars().collect();
    static ref BW_DIAC_CHARSET: HashSet<char> = "FKN`aiou~_".chars().collect();
    pub static ref BW_CHARSET: HashSet<char> = BW_LETTERS_CHARSET.union(&BW_DIAC_CHARSET).cloned().collect();

    static ref SAFEBW_LETTERS_CHARSET: HashSet<char> = "ABCDEGHIJLMOPQSTVWYZ_bcdefghjklmnpqrstvwxyz".chars().collect();
    static ref SAFEBW_DIAC_CHARSET: HashSet<char> = "FKNaeiou~_".chars().collect();
    pub static ref SAFEBW_CHARSET: HashSet<char> = SAFEBW_LETTERS_CHARSET.union(&SAFEBW_DIAC_CHARSET).cloned().collect();

    static ref XMLBW_LETTERS_CHARSET: HashSet<char> = "$'*ABDEGHIJOPSTWYZ_bdfghjklmnpqrstvwxyz{|}".chars().collect();
    static ref XMLBW_DIAC_CHARSET: HashSet<char> = "FKN`aiou~_".chars().collect();
    pub static ref XMLBW_CHARSET: HashSet<char> = XMLBW_LETTERS_CHARSET.union(&XMLBW_DIAC_CHARSET).cloned().collect();

    static ref HSB_LETTERS_CHARSET: HashSet<char> = "'ADHST_bcdfghjklmnpqrstvwxyzÂÄáðýĀĂĎħšŵŷγθς".chars().collect();
    static ref HSB_DIAC_CHARSET: HashSet<char> = ".aiu~Äáãĩũ_".chars().collect();
    pub static ref HSB_CHARSET: HashSet<char> = HSB_LETTERS_CHARSET.union(&HSB_DIAC_CHARSET).cloned().collect();
}
