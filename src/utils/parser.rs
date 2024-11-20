use regex::Regex;

/// Removes optional elements from Arabic text based on specified flags.
/// # Arguments
/// * `text` - The Arabic text to be processed.
/// * `diacs` - Remove Arabic diacritics (default: true).
/// * `small_diacs` - Remove small Quranic annotation signs (default: true).
/// * `shaddah` - Remove shaddah (default: true).
/// * `digit` - Remove Latin and Arabic digits (default: true).
/// * `alif` - Unify alif variations (default: true).
/// * `special_chars` - Remove special characters (default: true).
///
/// # Returns
/// Processed string with the specified elements removed.
pub fn ar_strip(
    text: &str,
    diacs: bool,
    small_diacs: bool,
    shaddah: bool,
    digit: bool,
    alif: bool,
    special_chars: bool,
) -> String {
    let mut result = text.to_string();

    if diacs {
        let diacs_re = Regex::new(r"[\u064B-\u0650]+").unwrap();
        let sukun_re = Regex::new(r"[\u0652]+").unwrap();
        result = diacs_re.replace_all(&result, "").into_owned();
        result = sukun_re.replace_all(&result, "").into_owned();
    }
    if shaddah {
        let shaddah_re = Regex::new(r"[\u0651]+").unwrap();
        result = shaddah_re.replace_all(&result, "").into_owned();
    }
    if small_diacs {
        let small_diacs_re = Regex::new(r"[\u06D6-\u06ED]+").unwrap();
        result = small_diacs_re.replace_all(&result, "").into_owned();
    }
    if digit {
        let latin_digits_re = Regex::new(r"[0-9]+").unwrap();
        let arabic_digits_re = Regex::new(r"[٠-٩]+").unwrap();
        result = latin_digits_re.replace_all(&result, " ").into_owned();
        result = arabic_digits_re.replace_all(&result, " ").into_owned();
    }
    if alif {
        let alif_variations = [("ٱ", "ا"), ("أ", "ا"), ("إ", "ا"), ("آ", "ا")];
        for (variant, replacement) in alif_variations {
            result = result.replace(variant, replacement);
        }
    }
    if special_chars {
        let special_chars_re = Regex::new(r"[?؟!@#$%-]+").unwrap();
        result = special_chars_re.replace_all(&result, "").into_owned();
    }

    let spaces_re = Regex::new(r"\s+").unwrap();
    result = spaces_re.replace_all(&result, " ").into_owned();
    result = result
        // remove underscore
        .replace("_", "")
        // remove arabic tatwelah
        .replace("ـ", "")
        .trim()
        .to_string();

    result
}

/// Removes Arabic and English punctuation marks from the input text.
pub fn remove_punctuation(text: &str) -> String {
    let punctuation_marks = vec![
        r"[\u0021-\u002F]+",
        r"[\u003A-\u0040]+",
        r"[\u005B-\u0060]+",
        r"[\u007B-\u007E]+",
        r"[\u060C]+",
        r"[\u061B]+",
        r"[\u061E]+",
        r"[\u061F]+",
        r"[\u0640]+",
        r"[\u0653]+",
        r"[\u065C]+",
        r"[\u066C]+",
        r"[\u066A]+",
        r#"["}"]+"#,
        r#"["{"]+"#,
    ];

    let mut result = text.to_string();
    for pattern in punctuation_marks {
        let re = Regex::new(pattern).unwrap();
        result = re.replace_all(&result, "").into_owned();
    }
    result
}

/// Removes all Latin letters from the input text.
pub fn remove_latin(text: &str) -> String {
    let latin_re = Regex::new(r"[a-zA-Z]+").unwrap();
    latin_re.replace_all(text, "").into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ar_strip_remove_diacritics() {
        let input = "أَلَمْ يُؤْمِنُوا";
        let expected = "ألم يؤمنوا";
        let result = ar_strip(input, true, false, false, false, false, false);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_ar_strip_remove_shaddah() {
        let input = "يحبّها";
        let expected = "يحبها";
        let result = ar_strip(input, false, false, true, false, false, false);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_ar_strip_remove_digits() {
        let input = "2023الجو جميلُ123";
        let expected = "الجو جميلُ";
        let result = ar_strip(input, false, false, false, true, false, false);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_ar_strip_unify_alif() {
        let input = "أَلِفْ آ إ";
        let expected = "اَلِفْ ا ا";
        let result = ar_strip(input, false, false, false, false, true, false);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_ar_strip_remove_special_chars() {
        let input = "!الجو؟ جميلُ#";
        let expected = "الجو جميلُ";
        let result = ar_strip(input, false, false, false, false, false, true);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_remove_punctuation() {
        let input = "te!@#،$%%؟st";
        let expected = "test";
        let result = remove_punctuation(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_remove_punctuation_arabic() {
        let input = "يَا أَيُّهَا الَّذِينَ آمَنُوا لِيَسْتَأْذِنْكُمُ ....";
        let expected = "يَا أَيُّهَا الَّذِينَ آمَنُوا لِيَسْتَأْذِنْكُمُ ";
        let result = remove_punctuation(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_remove_latin() {
        let input = "miojkdujhvaj1546545spkdpoqfoiehwv";
        let expected = "1546545";
        let result = remove_latin(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_remove_latin_with_arabic() {
        let input = "أصل المسمى Enterprise Resource Planning";
        let expected = "أصل المسمى   ";
        let result = remove_latin(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_ar_strip_combined() {
        let input = "أَلَمْ يُؤْمِنُوا 2023!؟";
        let expected = "الم يؤمنوا";
        let result = ar_strip(input, true, false, true, true, true, true);
        assert_eq!(result, expected);
    }
}
