use regex::Regex;

pub enum Language {
    English,
    French,
    German,
    Dutch,
    Polish,
    Bulgarian,
}

pub struct FleschReadingEase {
    language: Language,
}

impl FleschReadingEase {
    pub fn new(language: Language) -> Self {
        FleschReadingEase { language }
    }

    pub fn calculate(&self, text: &str) -> f64 {
        let sentence_count = self.count_sentences(text);
        let word_count = self.count_words(text);
        let syllable_count = self.count_syllables(text);

        if sentence_count == 0 || word_count == 0 {
            return 0.0;
        }

        let score = match self.language {
            Language::English => {
                206.835 - 1.015 * (word_count as f64 / sentence_count as f64) - 84.6 * (syllable_count as f64 / word_count as f64)
            },
            Language::French => {
                207.0 - 1.015 * (word_count as f64 / sentence_count as f64) - 73.6 * (syllable_count as f64 / word_count as f64)
            },
            Language::German => {
                180.0 - (word_count as f64 / sentence_count as f64) - 58.5 * (syllable_count as f64 / word_count as f64)
            },
            Language::Dutch => {
                206.835 - 0.93 * (word_count as f64 / sentence_count as f64) - 77.0 * (syllable_count as f64 / word_count as f64)
            },
            Language::Polish => {
                206.835 - 1.3 * (word_count as f64 / sentence_count as f64) - 85.6 * (syllable_count as f64 / word_count as f64)
            },
            Language::Bulgarian => {
                206.835 - 1.5 * (word_count as f64 / sentence_count as f64) - 60.0 * (syllable_count as f64 / word_count as f64)
            },
        };

        score.clamp(0.0, 100.0)
    }

    fn count_sentences(&self, text: &str) -> usize {
        let re = Regex::new(r"[.!?]").unwrap();
        re.find_iter(text).count()
    }

    fn count_words(&self, text: &str) -> usize {
        let re = Regex::new(r"\w+").unwrap();
        re.find_iter(text).count()
    }

    fn count_syllables(&self, text: &str) -> usize {
        let re = Regex::new(r"[aeiouyAEIOUY]+").unwrap();
        re.find_iter(text).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_english() {
        let fre = FleschReadingEase::new(Language::English);
        let text = "This is a test sentence. It is designed to check the Flesch Reading Ease score.";
        let score = fre.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_french() {
        let fre = FleschReadingEase::new(Language::French);
        let text = "Ceci est une phrase de test. Elle est conçue pour vérifier le score de lisibilité de Flesch.";
        let score = fre.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_german() {
        let fre = FleschReadingEase::new(Language::German);
        let text = "Dies ist ein Testsatz. Es soll die Flesch-Lesbarkeitsbewertung überprüfen.";
        let score = fre.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_dutch() {
        let fre = FleschReadingEase::new(Language::Dutch);
        let text = "Dit is een testzin. Het is ontworpen om de Flesch-leesgemakscore te controleren.";
        let score = fre.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_polish() {
        let fre = FleschReadingEase::new(Language::Polish);
        let text = "To jest zdanie testowe. Jest zaprojektowane, aby sprawdzić wynik łatwości czytania Flescha.";
        let score = fre.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_bulgarian() {
        let fre = FleschReadingEase::new(Language::Bulgarian);
        let text = "Това е тестово изречение. То е предназначено да провери оценката за четивност на Флеш.";
        let score = fre.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_edge_cases() {
        let fre = FleschReadingEase::new(Language::English);
        let text = "";
        let score = fre.calculate(text);
        assert_eq!(score, 0.0);

        let text = "A.";
        let score = fre.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }
}
