use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;

use crate::utils::{
    charsets::AR_CHARSET,
    parser::{ar_strip, remove_punctuation},
    tokenizers_words::simple_word_tokenize,
};

lazy_static! {
    static ref IS_AR_RE: Regex = {
        let charset: String = AR_CHARSET.iter().collect();
        Regex::new(&format!("^[{}]+$", regex::escape(&charset))).unwrap()
    };
}

// Mock dictionary for demonstration
lazy_static! {
    static ref DICTIONARY: HashMap<String, Vec<(i32, String, i32, String, String)>> = {
        let mut m = HashMap::new();
        m.insert(
            "ذهب".to_string(),
            vec![(
                82202,
                "ذَهَبَ".to_string(),
                202001617,
                "ذ ه ب".to_string(),
                "فعل ماضي".to_string(),
            )],
        );
        m
    };
}

#[derive(Serialize, Debug)]
pub struct MorphologicalAnalysis {
    pub token: String,
    pub lemma: Option<String>,
    pub lemma_id: Option<i32>,
    pub pos: Option<String>,
    pub root: Option<String>,
    pub frequency: i32,
}

impl Default for MorphologicalAnalysis {
    fn default() -> Self {
        Self {
            token: Default::default(),
            lemma: Default::default(),
            lemma_id: Default::default(),
            pos: Default::default(),
            root: Default::default(),
            frequency: Default::default(),
        }
    }
}

impl PartialEq for MorphologicalAnalysis {
    fn eq(&self, other: &Self) -> bool {
        self.token == other.token
            && self.lemma == other.lemma
            && self.lemma_id == other.lemma_id
            && self.pos == other.pos
            && self.root == other.root
            && self.frequency == other.frequency
    }
}

fn find_solution(token: &str, flag: &str) -> Vec<MorphologicalAnalysis> {
    if let Some(solutions) = DICTIONARY.get(token) {
        let selected_solutions = if flag == "1" {
            solutions.iter().take(1).collect::<Vec<_>>()
        } else {
            solutions.iter().collect::<Vec<_>>()
        };

        return selected_solutions
            .iter()
            .map(|(freq, lemma, lemma_id, root, pos)| MorphologicalAnalysis {
                token: token.to_string(),
                lemma: Some(lemma.clone()),
                lemma_id: Some(*lemma_id),
                root: Some(root.clone()),
                pos: Some(pos.clone()),
                frequency: *freq,
            })
            .collect();
    }
    vec![]
}

pub fn analyze(text: &str, task: &str, flag: &str) -> Vec<MorphologicalAnalysis> {
    let mut output_list = vec![];

    let tokens = simple_word_tokenize(text);

    for token in tokens {
        let stripped_token = ar_strip(&token, false, true, false, false, false, false);
        let unified_token = stripped_token.replace('ٱ', "ا");

        if unified_token.chars().all(char::is_numeric) {
            output_list.push(MorphologicalAnalysis {
                token: unified_token,
                lemma: None,
                lemma_id: None,
                pos: Some("رقم".to_string()),
                root: None,
                frequency: 0,
            });
        } else if remove_punctuation(&unified_token).is_empty() {
            output_list.push(MorphologicalAnalysis {
                token: unified_token,
                lemma: None,
                lemma_id: None,
                pos: Some("علامة ترقيم".to_string()),
                root: None,
                frequency: 0,
            });
        } else if !_is_ar(&unified_token) {
            output_list.push(MorphologicalAnalysis {
                token: unified_token,
                lemma: None,
                lemma_id: None,
                pos: Some("أجنبي".to_string()),
                root: None,
                frequency: 0,
            });
        } else {
            let mut result_token = find_solution(&unified_token, flag);

            if result_token.is_empty() {
                let stripped_al = unified_token.trim_start_matches('ا').to_string();
                if stripped_al.len() > 5 {
                    result_token = find_solution(&stripped_al, flag);
                }
            }

            if result_token.is_empty() {
                let replaced_ha = unified_token.replace('ه', "ة");
                result_token = find_solution(&replaced_ha, flag);
            }

            if result_token.is_empty() {
                let unified_alef =
                    ar_strip(&unified_token, false, false, false, false, true, false); // Unify Alef
                result_token = find_solution(&unified_alef, flag);
            }

            if result_token.is_empty() {
                let undiac_token = ar_strip(&unified_token, true, false, true, true, false, false); // diacs, shaddah, digit
                result_token = find_solution(&undiac_token, flag);
            }

            if result_token.is_empty() {
                let undiac_token = ar_strip(&unified_token, true, true, true, false, true, false); // diacs, smallDiacs, shaddah, alif
                result_token = find_solution(&undiac_token, flag);
            }

            if !result_token.is_empty() {
                output_list.extend(result_token);
            } else {
                output_list.push(MorphologicalAnalysis {
                    token: unified_token,
                    lemma: None,
                    lemma_id: None,
                    pos: None,
                    root: None,
                    frequency: 0,
                });
            }
        }
    }

    filter_results(output_list, task)
}

fn filter_results(data: Vec<MorphologicalAnalysis>, task: &str) -> Vec<MorphologicalAnalysis> {
    data.into_iter()
        .map(|item| match task {
            "lemmatization" => MorphologicalAnalysis {
                token: item.token,
                lemma: item.lemma,
                lemma_id: item.lemma_id,
                frequency: item.frequency,
                ..Default::default()
            },
            "pos" => MorphologicalAnalysis {
                token: item.token,
                pos: item.pos,
                frequency: item.frequency,
                ..Default::default()
            },
            "root" => MorphologicalAnalysis {
                token: item.token,
                root: item.root,
                frequency: item.frequency,
                ..Default::default()
            },
            _ => item,
        })
        .collect()
}

fn _is_ar(word: &str) -> bool {
    IS_AR_RE.is_match(word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_full_task() {
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
}
