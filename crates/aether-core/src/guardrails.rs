//! Zero-Guilt Philosophy (Sıfır Suçluluk Felsefesi) Guardrails.
//!
//! Ensures that neither AI outputs nor system notifications ever use guilt-inducing
//! language (`ihmal`, `unuttun`, `ceza`, `hata`, `geç kaldın`, `eksik`, `başarısız`).

use serde::{Deserialize, Serialize};

/// Official list of forbidden guilt-inducing terms and their compassionate replacements.
pub const FORBIDDEN_REPLACEMENTS: &[(&str, &str)] = &[
    ("ihmal", "dinlenme molası"),
    ("unuttun", "şimdi hatırladın"),
    ("ceza", "nazik denge"),
    ("hata", "öğrenme adımı"),
    ("geç kaldın", "tam zamanında buradasın"),
    ("eksik", "gelişmeye açık"),
    ("başarısız", "yeni bir deneyim"),
];

/// Report returned after scanning and sanitizing text through the Zero-Guilt Filter.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SanitizationReport {
    pub original: String,
    pub sanitized: String,
    pub replaced_terms: Vec<String>,
    pub is_clean: bool,
}

/// Zero-Guilt Filter that transforms punitive language into compassionate sanctuary prose.
#[derive(Debug, Clone, Default)]
pub struct ZeroGuiltFilter;

impl ZeroGuiltFilter {
    /// Checks if the given text is completely free of forbidden guilt-inducing words.
    pub fn is_compliant(text: &str) -> bool {
        let lower = text.to_lowercase();
        !FORBIDDEN_REPLACEMENTS
            .iter()
            .any(|(forbidden, _)| lower.contains(forbidden))
    }

    /// Sanitizes any text by replacing forbidden words with compassionate alternatives.
    pub fn sanitize(text: &str) -> SanitizationReport {
        let mut current = text.to_string();
        let mut replaced_terms = Vec::new();

        for &(forbidden, replacement) in FORBIDDEN_REPLACEMENTS {
            let lower = current.to_lowercase();
            if lower.contains(forbidden) {
                replaced_terms.push(forbidden.to_string());
                current = replace_case_insensitive(&current, forbidden, replacement);
            }
        }

        SanitizationReport {
            original: text.to_string(),
            is_clean: replaced_terms.is_empty(),
            sanitized: current,
            replaced_terms,
        }
    }
}

fn replace_case_insensitive(haystack: &str, needle: &str, replacement: &str) -> String {
    let lower_haystack = haystack.to_lowercase();
    let lower_needle = needle.to_lowercase();
    let mut result = String::with_capacity(haystack.len());
    let mut last_idx = 0;

    for (idx, _) in lower_haystack.match_indices(&lower_needle) {
        result.push_str(&haystack[last_idx..idx]);
        result.push_str(replacement);
        last_idx = idx + needle.len();
    }
    result.push_str(&haystack[last_idx..]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_guilt_filter_replaces_all_forbidden_words() {
        let raw = "Bugün odaklanmayı ihmal ettin ve unuttun, bu bir hata veya başarısız adım değil ama eksik kaldı ve geç kaldın, ceza yok.";
        assert!(!ZeroGuiltFilter::is_compliant(raw));

        let report = ZeroGuiltFilter::sanitize(raw);
        assert!(!report.is_clean);
        assert!(ZeroGuiltFilter::is_compliant(&report.sanitized));
        assert_eq!(report.replaced_terms.len(), 7);
    }

    #[test]
    fn test_clean_text_passes_unchanged() {
        let clean = "Zihnine berraklık, akışına dinginlik. Bugün 15 dakika odaklandın, harika bir adım.";
        assert!(ZeroGuiltFilter::is_compliant(clean));
        let report = ZeroGuiltFilter::sanitize(clean);
        assert!(report.is_clean);
        assert_eq!(report.sanitized, clean);
    }
}
