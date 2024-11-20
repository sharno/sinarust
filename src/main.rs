use morphology::morph_analyzer::{analyze, MorphologicalAnalysis};
use utils::{charsets::AR_CHARSET, tokenizers_words::TOKENIZE_RE};

pub mod morphology;
pub mod utils;

fn main() {
    lazy_static::initialize(&AR_CHARSET);
    lazy_static::initialize(&TOKENIZE_RE);

    let text = "ذهب";
    let result = analyze(text, "full", "1");

    let expected = vec![MorphologicalAnalysis {
        token: "ذهب".to_string(),
        lemma: Some("ذَهَبَ".to_string()),
        lemma_id: Some(202001617),
        pos: Some("فعل ماضي".to_string()),
        root: Some("ذ ه ب".to_string()),
        frequency: 82202,
    }];

    assert_eq!(result, expected);
}
