use std::sync::OnceLock;

use lingua::LanguageDetector;
use redis_module::RedisString;

use crate::config::LanguageOption;

static DETECTOR: OnceLock<LanguageDetector> = OnceLock::new();

/// Initializes the language detector.
pub(crate) fn init_detector(language_option: LanguageOption, low_accuracy_mode: bool) {
    let mut builder = match language_option {
        LanguageOption::AllLanguages => lingua::LanguageDetectorBuilder::from_all_languages(),
        LanguageOption::AllArabicScriptLanguages => {
            lingua::LanguageDetectorBuilder::from_all_languages_with_arabic_script()
        }
        LanguageOption::AllCyrillicScriptLanguages => {
            lingua::LanguageDetectorBuilder::from_all_languages_with_cyrillic_script()
        }
        LanguageOption::AllDevanagariScriptLanguages => {
            lingua::LanguageDetectorBuilder::from_all_languages_with_devanagari_script()
        }
        LanguageOption::AllLatinScriptLanguages => {
            lingua::LanguageDetectorBuilder::from_all_languages_with_latin_script()
        }
        LanguageOption::AllSpokenLanguages => {
            lingua::LanguageDetectorBuilder::from_all_spoken_languages()
        }
    };
    if low_accuracy_mode {
        builder.with_low_accuracy_mode();
    }
    DETECTOR.get_or_init(|| builder.build());
}

/// Runs the language detection on the given string.
pub(crate) fn run_language_detection(s: &RedisString) -> Option<RedisString> {
    DETECTOR
        .get()?
        .detect_language_of(s.to_string())
        .map(|lang| RedisString::create(None, lang.to_string()))
}
