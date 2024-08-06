use std::sync::OnceLock;

use redis_module::{enum_configuration, RedisGILGuard};

enum_configuration! {
    #[derive(Debug, Copy)]
    pub(crate) enum LanguageOption {
        AllLanguages = 1,
        AllSpokenLanguages = 2,
        AllArabicScriptLanguages = 3,
        AllCyrillicScriptLanguages = 4,
        AllDevanagariScriptLanguages = 5,
        AllLatinScriptLanguages = 6,
    }
}

/// Convenience function to get the current language option.
pub(crate) fn language_option() -> &'static RedisGILGuard<LanguageOption> {
    static CONFIGURATION_LANGUAGE_OPTION: OnceLock<RedisGILGuard<LanguageOption>> = OnceLock::new();
    CONFIGURATION_LANGUAGE_OPTION.get_or_init(|| RedisGILGuard::new(LanguageOption::AllLanguages))
}

/// Convenience function to get the current low accuracy mode.
pub(crate) fn low_accuracy_mode() -> &'static RedisGILGuard<bool> {
    static CONFIGURATION_LOW_ACCURACY_MODE: OnceLock<RedisGILGuard<bool>> = OnceLock::new();
    CONFIGURATION_LOW_ACCURACY_MODE.get_or_init(|| RedisGILGuard::new(false))
}
